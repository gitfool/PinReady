use eframe::egui;

use crate::audio::{self, AudioCommand, AudioConfig, Sound3DMode};
use crate::config::VpxConfig;
use crate::db::Database;
use crate::inputs::{self, CapturedInput, InputAction, JoystickEvent};
use crate::screens::{self, DisplayInfo, DisplayRole};
use crate::tilt::TiltConfig;

/// Wizard pages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardPage {
    Screens,
    Rendering,
    Inputs,
    Tilt,
    Audio,
    TablesDir,
}

impl WizardPage {
    fn title(&self) -> &'static str {
        match self {
            Self::Screens => "Ecrans",
            Self::Rendering => "Rendu",
            Self::Inputs => "Inputs",
            Self::Tilt => "Tilt / Nudge",
            Self::Audio => "Audio",
            Self::TablesDir => "Tables",
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::Screens => 0,
            Self::Rendering => 1,
            Self::Inputs => 2,
            Self::Tilt => 3,
            Self::Audio => 4,
            Self::TablesDir => 5,
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Screens),
            1 => Some(Self::Rendering),
            2 => Some(Self::Inputs),
            3 => Some(Self::Tilt),
            4 => Some(Self::Audio),
            5 => Some(Self::TablesDir),
            _ => None,
        }
    }

    fn count() -> usize {
        6
    }
}

/// State for input capture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaptureState {
    Idle,
    /// Waiting for input for action at this index
    Capturing(usize),
}

pub struct App {
    page: WizardPage,
    config: VpxConfig,
    db: Database,

    // Page 1 — Screens
    displays: Vec<DisplayInfo>,
    screen_count: usize,
    view_mode: i32, // 0=Desktop, 1=Cabinet, 2=FSS
    disable_touch: bool,

    // Cabinet physical dimensions (cm) for Window projection mode
    screen_inclination: f32,  // Playfield screen angle, 0 = horizontal
    lockbar_width: f32,       // Lockbar width in cm
    lockbar_height: f32,      // Lockbar height from ground in cm
    player_x: f32,            // Player X offset from center in cm
    player_y: f32,            // Player Y distance from lockbar in cm (negative = behind)
    player_z: f32,            // Player Z height (eyes) from playfield in cm

    // Page 2 — Rendering
    aa_factor: f32,        // Supersampling 0.5–2.0 (default 1.0)
    msaa: i32,             // 0=Off, 1=4x, 2=6x, 3=8x
    fxaa: i32,             // 0=Off, 1–7 various modes
    sharpen: i32,          // 0=Off, 1=CAS, 2=Bilateral CAS
    pf_reflection: i32,    // 0–5 reflection quality
    max_tex_dim: i32,      // 512–16384
    sync_mode: i32,        // 0=No sync, 1=VSync
    max_framerate: f32,    // -1=display, 0=unlimited, else value

    // Live accelerometer data from joystick thread
    accel_x: f32,
    accel_y: f32,

    // Page 3 — Inputs
    actions: Vec<InputAction>,
    capture_state: CaptureState,
    show_advanced_inputs: bool,
    joystick_rx: Option<crossbeam_channel::Receiver<JoystickEvent>>,

    // Page 3 — Tilt
    tilt: TiltConfig,

    // Page 4 — Audio
    audio: AudioConfig,
    audio_cmd_tx: Option<crossbeam_channel::Sender<AudioCommand>>,

    // Page 5 — Tables dir
    tables_dir: String,

    // Assets directory path
    #[allow(dead_code)]
    assets_dir: String,

    // Quit timer (set after finalize_wizard)
    quit_after_ms: Option<std::time::Instant>,
}

impl App {
    pub fn new(config: VpxConfig, db: Database) -> Self {
        // Enumerate displays
        let displays = screens::enumerate_displays();
        let screen_count = displays.len().min(4);

        // Auto-set cabinet mode for 2+ screens
        let view_mode = if screen_count >= 2 { 1 } else { 0 };
        let disable_touch = config.get_i32("Player", "NumberOfTimesToShowTouchMessage").unwrap_or(10) == 0;

        // Cabinet physical dimensions
        let screen_inclination = config.get_f32("Player", "ScreenInclination").unwrap_or(0.0);
        let lockbar_width = config.get_f32("Player", "LockbarWidth").unwrap_or(70.0);
        let lockbar_height = config.get_f32("Player", "LockbarHeight").unwrap_or(85.0);
        let player_x = config.get_f32("Player", "ScreenPlayerX").unwrap_or(0.0);
        let player_y = config.get_f32("Player", "ScreenPlayerY").unwrap_or(-10.0);
        let player_z = config.get_f32("Player", "ScreenPlayerZ").unwrap_or(70.0);

        // Load rendering config (pre-fill from ini)
        let aa_factor = config.get_f32("Player", "AAFactor").unwrap_or(1.0);
        let msaa = config.get_i32("Player", "MSAASamples").unwrap_or(0);
        let fxaa = config.get_i32("Player", "FXAA").unwrap_or(0);
        let sharpen = config.get_i32("Player", "Sharpen").unwrap_or(0);
        let pf_reflection = config.get_i32("Player", "PFReflection").unwrap_or(5);
        let max_tex_dim = config.get_i32("Player", "MaxTexDimension").unwrap_or(16384);
        let sync_mode = config.get_i32("Player", "SyncMode").unwrap_or(1);
        let max_framerate = config.get_f32("Player", "MaxFramerate").unwrap_or(-1.0);

        // Load input actions with defaults
        let mut actions = inputs::default_actions();
        // Pre-fill from existing config
        log::info!("Loading input mappings from ini...");
        for action in &mut actions {
            if let Some(mapping_str) = config.get_input_mapping(action.setting_id) {
                log::info!("  {} = {}", action.setting_id, mapping_str);
                // Parse simple keyboard mapping: "Key;<scancode>"
                if let Some(sc_str) = mapping_str.strip_prefix("Key;") {
                    if let Ok(sc_val) = sc_str.split('|').next().unwrap_or("").split('&').next().unwrap_or("").trim().parse::<i32>() {
                        let scancode = sdl3_sys::everything::SDL_Scancode(sc_val);
                        action.mapping = Some(CapturedInput::Keyboard {
                            scancode,
                            name: inputs::scancode_name(scancode),
                        });
                    }
                }
            }
        }

        // Load tilt config
        let mut tilt = TiltConfig::default();
        tilt.load_from_config(&config);

        // Load audio config
        let mut audio = AudioConfig::default();
        audio.load_from_config(&config);
        audio.available_devices = AudioConfig::enumerate_devices();

        // Load tables directory
        let tables_dir = db
            .get_tables_dir()
            .unwrap_or_default();

        // Determine assets directory (next to the binary or in project root)
        let assets_dir = {
            let exe_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            let candidate = exe_dir.join("assets").join("audio");
            if candidate.is_dir() {
                candidate.to_string_lossy().into_owned()
            } else {
                // Fallback: current working directory
                let cwd = std::env::current_dir().unwrap_or_default();
                cwd.join("assets").join("audio").to_string_lossy().into_owned()
            }
        };

        // Start joystick thread (keyboard captured via egui)
        let joystick_rx = inputs::spawn_joystick_thread();

        // Open audio stream on main thread (required by some platforms)
        let audio_stream = audio::open_audio_stream();
        let audio_cmd_tx = audio::spawn_audio_thread(audio_stream, assets_dir.clone());

        Self {
            page: WizardPage::Screens,
            config,
            db,
            displays,
            screen_count,
            view_mode,
            disable_touch,
            screen_inclination,
            lockbar_width,
            lockbar_height,
            player_x,
            player_y,
            player_z,
            actions,
            accel_x: 0.0,
            accel_y: 0.0,
            aa_factor,
            msaa,
            fxaa,
            sharpen,
            pf_reflection,
            max_tex_dim,
            sync_mode,
            max_framerate,
            capture_state: CaptureState::Idle,
            show_advanced_inputs: false,
            joystick_rx: Some(joystick_rx),
            tilt,
            audio,
            audio_cmd_tx: Some(audio_cmd_tx),
            tables_dir,
            assets_dir,
            quit_after_ms: None,
        }
    }

    fn next_page(&mut self) {
        let next = self.page.index() + 1;
        if let Some(page) = WizardPage::from_index(next) {
            self.save_current_page();
            self.page = page;
        }
    }

    fn prev_page(&mut self) {
        if self.page.index() > 0 {
            if let Some(page) = WizardPage::from_index(self.page.index() - 1) {
                self.page = page;
            }
        }
    }

    fn save_current_page(&mut self) {
        match self.page {
            WizardPage::Screens => self.save_screens(),
            WizardPage::Rendering => self.save_rendering(),
            WizardPage::Inputs => self.save_inputs(),
            WizardPage::Tilt => self.save_tilt(),
            WizardPage::Audio => self.save_audio(),
            WizardPage::TablesDir => self.save_tables_dir(),
        }
    }

    fn save_screens(&mut self) {
        self.config.set_view_mode(self.view_mode);

        if self.disable_touch {
            self.config.set_i32("Player", "TouchOverlay", 0);
            self.config.set_i32("Player", "NumberOfTimesToShowTouchMessage", 0);
        }

        // Disable outputs that are not used based on screen count
        let has_backglass = self.displays.iter().any(|d| d.role == DisplayRole::Backglass);
        let has_dmd = self.displays.iter().any(|d| d.role == DisplayRole::Dmd);
        let has_topper = self.displays.iter().any(|d| d.role == DisplayRole::Topper);

        if !has_backglass {
            self.config.set_i32("Backglass", "BackglassOutput", 0); // Disabled
        }
        if !has_dmd {
            self.config.set_i32("ScoreView", "ScoreViewOutput", 0); // Disabled
            // DMD overlaid on backglass with auto-position
            if has_backglass {
                self.config.set_i32("Plugin.B2SLegacy", "BackglassDMDOverlay", 1);
                self.config.set_i32("Plugin.B2SLegacy", "BackglassDMDAutoPos", 1);
            }
        } else {
            // DMD has its own screen — hide grill from backglass, enable DMD overlay on ScoreView
            self.config.set_i32("Plugin.B2SLegacy", "B2SHideGrill", 1);
            self.config.set_i32("Plugin.B2SLegacy", "ScoreViewDMDOverlay", 1);
            self.config.set_i32("Plugin.B2SLegacy", "ScoreViewDMDAutoPos", 1);
            self.config.set_i32("Plugin.B2SLegacy", "BackglassDMDOverlay", 0);
        }
        if !has_topper {
            self.config.set_i32("Topper", "TopperOutput", 0); // Disabled
        }

        let placements = screens::compute_placement(&self.displays);
        for (i, display) in self.displays.iter().enumerate() {
            let (px, py) = placements[i];
            match display.role {
                DisplayRole::Playfield => {
                    self.config.set_playfield_display(&display.name, px, py, display.width, display.height);
                    // Physical screen size in cm for Window projection (mm -> cm, width > height)
                    let w_cm = display.width_mm as f32 / 10.0;
                    let h_cm = display.height_mm as f32 / 10.0;
                    if w_cm > 0.0 && h_cm > 0.0 {
                        // VPX expects width > height (landscape dimensions)
                        let (screen_w, screen_h) = if w_cm >= h_cm { (w_cm, h_cm) } else { (h_cm, w_cm) };
                        self.config.set_f32("Player", "ScreenWidth", screen_w);
                        self.config.set_f32("Player", "ScreenHeight", screen_h);
                    }
                }
                DisplayRole::Backglass => {
                    self.config.set_backglass_display(&display.name, px, py, display.width, display.height);
                }
                DisplayRole::Dmd => {
                    self.config.set_dmd_display(&display.name, px, py, display.width, display.height);
                }
                DisplayRole::Topper => {
                    self.config.set_topper_display(&display.name, px, py, display.width, display.height);
                }
                DisplayRole::Unused => {}
            }
        }

        // Cabinet physical dimensions
        self.config.set_f32("Player", "ScreenInclination", self.screen_inclination);
        self.config.set_f32("Player", "LockbarWidth", self.lockbar_width);
        self.config.set_f32("Player", "LockbarHeight", self.lockbar_height);
        self.config.set_f32("Player", "ScreenPlayerX", self.player_x);
        self.config.set_f32("Player", "ScreenPlayerY", self.player_y);
        self.config.set_f32("Player", "ScreenPlayerZ", self.player_z);

        if let Err(e) = self.config.save() {
            log::error!("Failed to save config: {e}");
        }
    }

    fn save_rendering(&mut self) {
        self.config.set_f32("Player", "AAFactor", self.aa_factor);
        self.config.set_i32("Player", "MSAASamples", self.msaa);
        self.config.set_i32("Player", "FXAA", self.fxaa);
        self.config.set_i32("Player", "Sharpen", self.sharpen);
        self.config.set_i32("Player", "PFReflection", self.pf_reflection);
        self.config.set_i32("Player", "MaxTexDimension", self.max_tex_dim);
        self.config.set_i32("Player", "SyncMode", self.sync_mode);
        self.config.set_f32("Player", "MaxFramerate", self.max_framerate);
        if let Err(e) = self.config.save() {
            log::error!("Failed to save config: {e}");
        }
    }

    fn save_inputs(&mut self) {
        for action in &self.actions {
            let mapping = match &action.mapping {
                Some(captured) => captured.to_mapping_string(),
                None => {
                    if action.default_scancode == sdl3_sys::everything::SDL_SCANCODE_UNKNOWN {
                        continue; // No default, no mapping
                    }
                    format!("Key;{}", action.default_scancode.0)
                }
            };
            self.config.set_input_mapping(action.setting_id, &mapping);
        }

        if let Err(e) = self.config.save() {
            log::error!("Failed to save config: {e}");
        }
    }

    fn save_tilt(&mut self) {
        self.tilt.save_to_config(&mut self.config);
        if let Err(e) = self.config.save() {
            log::error!("Failed to save config: {e}");
        }
    }

    fn save_audio(&mut self) {
        self.audio.save_to_config(&mut self.config);
        if let Err(e) = self.config.save() {
            log::error!("Failed to save config: {e}");
        }
    }

    fn save_tables_dir(&mut self) {
        if let Err(e) = self.db.set_tables_dir(&self.tables_dir) {
            log::error!("Failed to save tables dir: {e}");
        }
    }

    fn finalize_wizard(&mut self, _ctx: &egui::Context) {
        // Save ALL pages
        self.save_screens();
        self.save_rendering();
        self.save_inputs();
        self.save_tilt();
        self.save_audio();
        self.save_tables_dir();

        if let Err(e) = self.db.set_configured() {
            log::error!("Failed to mark wizard complete: {e}");
        }

        // Knocker surprise!
        if let Some(tx) = &self.audio_cmd_tx {
            let _ = tx.send(AudioCommand::PlayOnSpeaker {
                path: "knocker.ogg".to_string(),
                target: audio::SpeakerTarget::FrontBoth,
            });
        }

        log::info!("Wizard completed! Config saved to VPinballX.ini");

        // Set flag to quit after knocker plays
        self.quit_after_ms = Some(std::time::Instant::now());
    }

    // --- Page rendering ---

    fn render_screens_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configuration des écrans");
        ui.add_space(8.0);

        // Screen count selection
        ui.label("Nombre d'écrans pour votre cabinet :");
        ui.horizontal(|ui| {
            for n in 1..=4 {
                let label = match n {
                    1 => "1 écran",
                    2 => "2 écrans",
                    3 => "3 écrans",
                    _ => "4 écrans",
                };
                if ui.radio_value(&mut self.screen_count, n, label).changed() {
                    // Re-assign roles based on new screen count
                    for (i, display) in self.displays.iter_mut().enumerate() {
                        let roles = [DisplayRole::Playfield, DisplayRole::Backglass, DisplayRole::Dmd, DisplayRole::Topper];
                        display.role = if i < self.screen_count {
                            roles.get(i).copied().unwrap_or(DisplayRole::Unused)
                        } else {
                            DisplayRole::Unused
                        };
                    }
                }
            }
        });

        ui.add_space(8.0);

        // View mode
        ui.label("Mode d'affichage :");
        ui.horizontal(|ui| {
            ui.radio_value(&mut self.view_mode, 0, "Desktop");
            ui.radio_value(&mut self.view_mode, 1, "Cabinet");
            ui.radio_value(&mut self.view_mode, 2, "Full Single Screen");
        });

        ui.add_space(8.0);

        ui.checkbox(&mut self.disable_touch, "Desactiver l'ecran tactile (rustdesk, VNC...)");

        ui.add_space(12.0);

        // Display table
        if self.displays.is_empty() {
            ui.label("Aucun écran détecté.");
            return;
        }

        egui::Grid::new("displays_grid")
            .striped(true)
            .min_col_width(80.0)
            .show(ui, |ui| {
                ui.strong("Ecran");
                ui.strong("Resolution");
                ui.strong("Hz");
                ui.strong("Taille physique");
                ui.strong("Role");
                ui.end_row();

                let available_roles: Vec<DisplayRole> = DisplayRole::all()
                    .iter()
                    .copied()
                    .filter(|r| {
                        *r == DisplayRole::Unused || {
                            let needed = match r {
                                DisplayRole::Playfield => self.screen_count >= 1,
                                DisplayRole::Backglass => self.screen_count >= 2,
                                DisplayRole::Dmd => self.screen_count >= 3,
                                DisplayRole::Topper => self.screen_count >= 4,
                                DisplayRole::Unused => true,
                            };
                            needed
                        }
                    })
                    .collect();

                for display in &mut self.displays {
                    // Name + inches
                    let label = if let Some(inches) = display.size_inches {
                        format!("{} ({}\")", display.name, inches)
                    } else {
                        display.name.clone()
                    };
                    ui.label(&label);
                    ui.label(format!("{}x{}", display.width, display.height));
                    ui.label(format!("{:.0} Hz", display.refresh_rate));
                    // Physical size mm (editable)
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut display.width_mm).speed(1).suffix(" mm"));
                        ui.label("x");
                        ui.add(egui::DragValue::new(&mut display.height_mm).speed(1).suffix(" mm"));
                    });

                    egui::ComboBox::from_id_salt(format!("role_{}", display.name))
                        .selected_text(display.role.label())
                        .show_ui(ui, |ui| {
                            for role in &available_roles {
                                ui.selectable_value(&mut display.role, *role, role.label());
                            }
                        });
                    ui.end_row();
                }
            });

        ui.add_space(16.0);
        ui.strong("Dimensions physiques du cabinet (pour la projection 3D)");
        ui.add_space(4.0);
        ui.label("Ces mesures permettent a VPX de calculer la perspective comme si vous regardiez un vrai flipper.");
        ui.add_space(8.0);

        ui.label("Repere : l'origine est au centre du bord bas du playfield (cote lockbar/joueur).");
        ui.label("X = gauche/droite, Y = avant/arriere, Z = haut/bas.");
        ui.add_space(8.0);

        egui::Grid::new("cabinet_dims")
            .min_col_width(350.0)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Inclinaison du playfield (0 = a plat, positif = releve cote backglass) :");
                ui.add(egui::Slider::new(&mut self.screen_inclination, -30.0..=30.0).suffix(" deg"));
                ui.end_row();

                ui.label("Largeur de la lockbar (barre metallique en facade du cab, en cm) :");
                ui.add(egui::Slider::new(&mut self.lockbar_width, 10.0..=150.0).suffix(" cm"));
                ui.end_row();

                ui.label("Hauteur de la lockbar par rapport au sol (en cm) :");
                ui.add(egui::Slider::new(&mut self.lockbar_height, 0.0..=250.0).suffix(" cm"));
                ui.end_row();

                ui.label("Position du joueur X (0 = centre, negatif = gauche, positif = droite) :");
                ui.add(egui::Slider::new(&mut self.player_x, -30.0..=30.0).suffix(" cm"));
                ui.end_row();

                ui.label("Position du joueur Y (distance ventre-lockbar, negatif = en retrait) :");
                ui.add(egui::Slider::new(&mut self.player_y, -70.0..=30.0).suffix(" cm"));
                ui.end_row();

                ui.label("Position du joueur Z (hauteur des yeux au dessus du bord bas du playfield) :");
                ui.add(egui::Slider::new(&mut self.player_z, 30.0..=100.0).suffix(" cm"));
                ui.end_row();
            });
    }

    fn render_rendering_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Rendu Playfield");
        ui.add_space(4.0);
        ui.label("Parametres de rendu graphique. Recommandation: commencez par les valeurs par defaut et ajustez selon les performances.");
        ui.add_space(12.0);

        egui::Grid::new("rendering_grid")
            .min_col_width(250.0)
            .striped(true)
            .show(ui, |ui| {
                // Sync mode
                ui.label("Synchronisation :");
                egui::ComboBox::from_id_salt("sync_mode")
                    .selected_text(match self.sync_mode {
                        0 => "Pas de sync",
                        1 => "VSync (recommande)",
                        _ => "VSync",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sync_mode, 0, "Pas de sync");
                        ui.selectable_value(&mut self.sync_mode, 1, "VSync (defaut, recommande)");
                    });
                ui.end_row();

                // Max framerate
                ui.label("Limite FPS :");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.max_framerate, -1.0..=240.0));
                    ui.label(match self.max_framerate as i32 {
                        -1 => "(= refresh ecran)".to_string(),
                        0 => "(illimite)".to_string(),
                        v => format!("{} fps", v),
                    });
                });
                ui.end_row();

                // Supersampling
                ui.label("Supersampling (AA Factor) :");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.aa_factor, 0.5..=2.0).step_by(0.25));
                    let tip = if self.aa_factor < 0.8 {
                        "Performance++"
                    } else if self.aa_factor <= 1.1 {
                        "Defaut"
                    } else if self.aa_factor <= 1.5 {
                        "Qualite+"
                    } else {
                        "Qualite++ (lourd)"
                    };
                    ui.label(tip);
                });
                ui.end_row();

                // MSAA
                ui.label("MSAA :");
                egui::ComboBox::from_id_salt("msaa")
                    .selected_text(match self.msaa {
                        0 => "Desactive",
                        1 => "4 Samples",
                        2 => "6 Samples",
                        3 => "8 Samples",
                        _ => "Desactive",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.msaa, 0, "Desactive (defaut)");
                        ui.selectable_value(&mut self.msaa, 1, "4 Samples");
                        ui.selectable_value(&mut self.msaa, 2, "6 Samples");
                        ui.selectable_value(&mut self.msaa, 3, "8 Samples");
                    });
                ui.end_row();

                // Post-process AA
                ui.label("Anti-aliasing post-traitement :");
                egui::ComboBox::from_id_salt("fxaa")
                    .selected_text(match self.fxaa {
                        0 => "Desactive",
                        1 => "Fast FXAA",
                        2 => "Standard FXAA",
                        3 => "Quality FXAA",
                        4 => "Fast NFAA",
                        5 => "Standard DLAA",
                        6 => "Quality SMAA",
                        7 => "Quality FAAA",
                        _ => "Desactive",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.fxaa, 0, "Desactive (defaut)");
                        ui.selectable_value(&mut self.fxaa, 1, "Fast FXAA");
                        ui.selectable_value(&mut self.fxaa, 2, "Standard FXAA (recommande)");
                        ui.selectable_value(&mut self.fxaa, 3, "Quality FXAA");
                        ui.selectable_value(&mut self.fxaa, 4, "Fast NFAA");
                        ui.selectable_value(&mut self.fxaa, 5, "Standard DLAA");
                        ui.selectable_value(&mut self.fxaa, 6, "Quality SMAA");
                        ui.selectable_value(&mut self.fxaa, 7, "Quality FAAA");
                    });
                ui.end_row();

                // Sharpening
                ui.label("Nettete :");
                egui::ComboBox::from_id_salt("sharpen")
                    .selected_text(match self.sharpen {
                        0 => "Desactive",
                        1 => "CAS",
                        2 => "Bilateral CAS",
                        _ => "Desactive",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sharpen, 0, "Desactive (defaut)");
                        ui.selectable_value(&mut self.sharpen, 1, "CAS (recommande)");
                        ui.selectable_value(&mut self.sharpen, 2, "Bilateral CAS");
                    });
                ui.end_row();

                // Reflections
                ui.label("Qualite des reflets :");
                egui::ComboBox::from_id_salt("pf_reflection")
                    .selected_text(match self.pf_reflection {
                        0 => "Desactive",
                        1 => "Billes uniquement",
                        2 => "Statique",
                        3 => "Statique + Billes",
                        4 => "Statique + Dynamique (async)",
                        5 => "Dynamique (recommande)",
                        _ => "Dynamique",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.pf_reflection, 0, "Desactive (performances++)");
                        ui.selectable_value(&mut self.pf_reflection, 1, "Billes uniquement");
                        ui.selectable_value(&mut self.pf_reflection, 2, "Statique (zero cout)");
                        ui.selectable_value(&mut self.pf_reflection, 3, "Statique + Billes");
                        ui.selectable_value(&mut self.pf_reflection, 4, "Statique + Dynamique (async)");
                        ui.selectable_value(&mut self.pf_reflection, 5, "Dynamique (defaut, recommande)");
                    });
                ui.end_row();

                // Max texture dimension
                ui.label("Taille max textures :");
                egui::ComboBox::from_id_salt("max_tex")
                    .selected_text(format!("{}", self.max_tex_dim))
                    .show_ui(ui, |ui| {
                        for &size in &[512, 1024, 2048, 4096, 8192, 16384] {
                            let label = if size == 16384 { "16384 (defaut, recommande)".to_string() } else { format!("{size}") };
                            ui.selectable_value(&mut self.max_tex_dim, size, label);
                        }
                    });
                ui.end_row();
            });
    }

    fn render_inputs_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configuration des inputs");
        ui.add_space(4.0);
        ui.label("Cliquez sur \"Mapper\" puis appuyez sur une touche ou un bouton. Echap = garder la valeur actuelle.");
        ui.add_space(8.0);

        // Process keyboard input via egui (has window focus)
        if let CaptureState::Capturing(idx) = self.capture_state {
            // Check for modifier-only presses (Shift, Ctrl, Alt)
            let modifiers = ui.input(|i| i.modifiers);
            let mut captured = false;

            // Check key events
            let events: Vec<egui::Event> = ui.input(|i| i.events.clone());
            for event in &events {
                if let egui::Event::Key { key, pressed: true, .. } = event {
                    if *key == egui::Key::Escape {
                        self.capture_state = CaptureState::Idle;
                        captured = true;
                        break;
                    }
                    if let Some(sc) = inputs::egui_key_to_scancode(*key) {
                        if idx < self.actions.len() {
                            self.actions[idx].mapping = Some(CapturedInput::Keyboard {
                                scancode: sc,
                                name: inputs::scancode_name(sc),
                            });
                        }
                        self.capture_state = CaptureState::Idle;
                        captured = true;
                        break;
                    }
                }
            }

            // Check modifier-only press (e.g., just Shift pressed alone)
            if !captured && (modifiers.shift || modifiers.ctrl || modifiers.alt) {
                // Wait for a key event to pair with the modifier, or capture modifier alone
                // We only capture modifier alone if no other key event came through
                if events.is_empty() {
                    if let Some(sc) = inputs::egui_modifiers_to_scancode(&modifiers) {
                        if idx < self.actions.len() {
                            self.actions[idx].mapping = Some(CapturedInput::Keyboard {
                                scancode: sc,
                                name: inputs::scancode_name(sc),
                            });
                        }
                        self.capture_state = CaptureState::Idle;
                    }
                }
            }

            // Joystick events are processed in the main ui() method

            // Request repaint while capturing to stay responsive
            ui.ctx().request_repaint();
        }

        // Conflicts
        let conflicts = inputs::find_conflicts(&self.actions);

        // Essential actions
        ui.strong("Actions essentielles");
        self.render_action_list(ui, true, &conflicts);

        ui.add_space(8.0);
        ui.checkbox(&mut self.show_advanced_inputs, "Afficher les actions avancées");
        if self.show_advanced_inputs {
            ui.add_space(4.0);
            ui.strong("Actions avancées");
            self.render_action_list(ui, false, &conflicts);
        }
    }

    fn render_action_list(&mut self, ui: &mut egui::Ui, essential: bool, conflicts: &[(usize, usize)]) {
        egui::Grid::new(if essential { "essential_inputs" } else { "advanced_inputs" })
            .striped(true)
            .min_col_width(120.0)
            .show(ui, |ui| {
                ui.strong("Action");
                ui.strong("Touche actuelle");
                ui.strong("");
                ui.end_row();

                for (idx, action) in self.actions.iter().enumerate() {
                    if action.essential != essential {
                        continue;
                    }

                    ui.label(action.label);

                    // Current binding display
                    let is_capturing = self.capture_state == CaptureState::Capturing(idx);
                    let binding_text = if is_capturing {
                        "[...] Appuyez sur une touche...".to_string()
                    } else if let Some(captured) = &action.mapping {
                        captured.display_name().to_string()
                    } else if action.default_scancode != sdl3_sys::everything::SDL_SCANCODE_UNKNOWN {
                        format!("{} (défaut)", inputs::scancode_name(action.default_scancode))
                    } else {
                        "Non assigné".to_string()
                    };

                    // Conflict warning
                    let has_conflict = conflicts.iter().any(|(a, b)| *a == idx || *b == idx);
                    if has_conflict {
                        ui.colored_label(egui::Color32::from_rgb(255, 165, 0), format!("/!\\ {binding_text}"));
                    } else {
                        ui.label(&binding_text);
                    }

                    // Capture button
                    let btn_label = if is_capturing { "Annuler" } else { "Mapper" };
                    if ui.button(btn_label).clicked() {
                        if is_capturing {
                            self.capture_state = CaptureState::Idle;
                        } else {
                            self.capture_state = CaptureState::Capturing(idx);
                        }
                    }
                    ui.end_row();
                }
            });
    }

    fn render_tilt_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sensibilite Tilt / Nudge");
        ui.add_space(4.0);
        ui.label("Reglage de la sensibilite de votre accelerometre.");
        ui.add_space(12.0);

        // Request repaint for live accelerometer data
        ui.ctx().request_repaint();

        // Nudge sensitivity — how much the screen shakes when you push the cab
        ui.label("Nudge - intensite de l'effet visuel quand vous poussez la caisse :");
        let slider = egui::Slider::new(&mut self.tilt.nudge_sensitivity, 0.0..=1.0)
            .custom_formatter(|v, _| format!("{:.0}%", v * 100.0));
        if ui.add_sized([ui.available_width(), 24.0], slider).changed() && !self.tilt.advanced_mode {
            self.tilt.apply_nudge_sensitivity();
        }

        ui.add_space(12.0);

        // Tilt sensitivity — how easy it is to trigger TILT (game over)
        ui.label("Tilt - plus c'est haut, plus le TILT se declenche facilement :");
        let slider = egui::Slider::new(&mut self.tilt.tilt_sensitivity, 0.0..=1.0)
            .custom_formatter(|v, _| format!("{:.0}%", v * 100.0));
        if ui.add_sized([ui.available_width(), 24.0], slider).changed() && !self.tilt.advanced_mode {
            self.tilt.apply_tilt_sensitivity();
        }

        ui.add_space(16.0);

        // Accelerometer visualization
        ui.label("Accelerometre en direct - le point vert = position actuelle, cercle rouge = seuil TILT :");
        ui.add_space(4.0);
        let viz_size = egui::vec2(240.0, 240.0);
        let (rect, _response) = ui.allocate_exact_size(viz_size, egui::Sense::hover());
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = 110.0;

        // Outer circle (max range)
        painter.circle_stroke(center, radius, egui::Stroke::new(2.0, egui::Color32::GRAY));
        // Cross hairs
        painter.line_segment(
            [center - egui::vec2(radius, 0.0), center + egui::vec2(radius, 0.0)],
            egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
        );
        painter.line_segment(
            [center - egui::vec2(0.0, radius), center + egui::vec2(0.0, radius)],
            egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
        );

        // TILT threshold circle (red) — if the green dot touches this, it's TILT
        let threshold_radius = radius * (self.tilt.plumb_threshold_angle / 60.0);
        painter.circle_stroke(center, threshold_radius, egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 80, 80)));
        painter.text(
            center + egui::vec2(threshold_radius + 4.0, -10.0),
            egui::Align2::LEFT_CENTER, "TILT",
            egui::FontId::proportional(12.0), egui::Color32::from_rgb(255, 80, 80),
        );

        // Live accelerometer dot
        let dot_x = center.x + self.accel_x * radius;
        let dot_y = center.y + self.accel_y * radius;
        let dot_pos = egui::pos2(dot_x, dot_y);
        let dist = ((dot_x - center.x).powi(2) + (dot_y - center.y).powi(2)).sqrt();
        let dot_color = if dist > threshold_radius {
            egui::Color32::from_rgb(255, 50, 50) // in TILT zone
        } else {
            egui::Color32::from_rgb(100, 220, 100) // safe
        };
        painter.circle_filled(dot_pos, 7.0, dot_color);

        ui.add_space(12.0);
        ui.checkbox(&mut self.tilt.advanced_mode, "Mode avance");

        if self.tilt.advanced_mode {
            ui.add_space(8.0);

            ui.label("Nudge Strength (intensite de l'effet visuel) :");
            ui.add_sized([ui.available_width(), 20.0],
                egui::Slider::new(&mut self.tilt.nudge_strength, 0.0..=0.25));

            ui.label("Plumb Inertia (inertie du plomb de simulation) :");
            ui.add_sized([ui.available_width(), 20.0],
                egui::Slider::new(&mut self.tilt.plumb_inertia, 0.001..=1.0));

            ui.label("Plumb Threshold Angle (angle de declenchement du TILT) :");
            ui.add_sized([ui.available_width(), 20.0],
                egui::Slider::new(&mut self.tilt.plumb_threshold_angle, 5.0..=60.0).suffix(" deg"));

            ui.add_space(4.0);
            ui.checkbox(&mut self.tilt.nudge_filter_0, "Filtre anti-bruit capteur 1");
            ui.checkbox(&mut self.tilt.nudge_filter_1, "Filtre anti-bruit capteur 2");
        }
    }

    fn render_audio_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configuration audio");
        ui.add_space(8.0);

        // Device assignment
        ui.strong("Assignation des périphériques audio");
        ui.add_space(4.0);

        egui::Grid::new("audio_devices")
            .min_col_width(150.0)
            .show(ui, |ui| {
                ui.label("Backglass (musique, voix) :");
                egui::ComboBox::from_id_salt("device_bg")
                    .selected_text(if self.audio.device_bg.is_empty() { "Par défaut" } else { &self.audio.device_bg })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.audio.device_bg, String::new(), "Par défaut");
                        for dev in &self.audio.available_devices {
                            ui.selectable_value(&mut self.audio.device_bg, dev.clone(), dev);
                        }
                    });
                ui.end_row();

                ui.label("Playfield (sons mécaniques) :");
                egui::ComboBox::from_id_salt("device_pf")
                    .selected_text(if self.audio.device_pf.is_empty() { "Par défaut" } else { &self.audio.device_pf })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.audio.device_pf, String::new(), "Par défaut");
                        for dev in &self.audio.available_devices {
                            ui.selectable_value(&mut self.audio.device_pf, dev.clone(), dev);
                        }
                    });
                ui.end_row();
            });

        ui.add_space(12.0);

        // Sound3D mode
        ui.strong("Mode de sortie playfield");
        ui.add_space(4.0);
        for mode in Sound3DMode::all() {
            ui.radio_value(&mut self.audio.sound_3d_mode, *mode, mode.label());
        }

        // Wiring guide based on selected mode
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.strong("Cablage requis pour ce mode :");
        ui.add_space(4.0);

        match self.audio.sound_3d_mode {
            Sound3DMode::FrontStereo | Sound3DMode::RearStereo => {
                ui.label("Carte son : Stereo (2 canaux)");
                ui.label("  Sortie Vert (Front) -> Enceintes backglass (musique)");
                ui.label("  Playfield : le son sort sur les memes enceintes, pas de spatialisation");
            }
            Sound3DMode::SurroundRearLockbar => {
                ui.label("Carte son : 5.1 (6 canaux)");
                ui.label("  Sortie Vert  (Front L/R) -> Exciters playfield haut (cote backglass)");
                ui.label("  Sortie Noir  (Rear L/R)  -> Exciters playfield bas (cote lockbar/joueur)");
                ui.label("  Sortie Orange (Center/Sub) -> Caisson de basse (optionnel)");
                ui.label("  Backglass sur un device audio separe");
            }
            Sound3DMode::SurroundFrontLockbar => {
                ui.label("Carte son : 5.1 (6 canaux)");
                ui.label("  Sortie Vert  (Front L/R) -> Exciters playfield bas (cote lockbar/joueur)");
                ui.label("  Sortie Noir  (Rear L/R)  -> Exciters playfield haut (cote backglass)");
                ui.label("  Sortie Orange (Center/Sub) -> Caisson de basse (optionnel)");
                ui.label("  Backglass sur un device audio separe");
            }
            Sound3DMode::SsfLegacy | Sound3DMode::SsfNew => {
                ui.label("Carte son : 7.1 (8 canaux) -- Configuration SSF recommandee");
                ui.add_space(4.0);
                egui::Grid::new("wiring_grid")
                    .striped(true)
                    .min_col_width(120.0)
                    .show(ui, |ui| {
                        ui.strong("Sortie jack");
                        ui.strong("Canal");
                        ui.strong("Branchement pincab");
                        ui.end_row();

                        ui.label("Vert (Front)");
                        ui.label("FL / FR");
                        ui.label("Enceintes backglass (musique) ou systeme 2.1");
                        ui.end_row();

                        ui.label("Noir (Rear)");
                        ui.label("BL / BR");
                        ui.label("Exciters playfield haut (cote backglass)");
                        ui.end_row();

                        ui.label("Gris (Side)");
                        ui.label("SL / SR");
                        ui.label("Exciters playfield bas (cote lockbar/joueur)");
                        ui.end_row();

                        ui.label("Orange (Center/Sub)");
                        ui.label("FC / LFE");
                        ui.label("Caisson de basse / Bass shaker (optionnel)");
                        ui.end_row();
                    });
                ui.add_space(4.0);
                ui.label("Note : les couleurs des jacks peuvent varier selon la carte son !");
            }
        }

        ui.add_space(12.0);

        // Volumes
        ui.strong("Volumes");
        ui.add_space(4.0);
        egui::Grid::new("audio_volumes")
            .min_col_width(150.0)
            .show(ui, |ui| {
                ui.label("Musique (Backglass) :");
                ui.add(egui::Slider::new(&mut self.audio.music_volume, 0..=100).suffix("%"));
                ui.end_row();

                ui.label("Sons (Playfield) :");
                ui.add(egui::Slider::new(&mut self.audio.sound_volume, 0..=100).suffix("%"));
                ui.end_row();
            });

        ui.add_space(12.0);

        // === Tests audio ===
        ui.strong("Test 1 - Musique Backglass (Front L/R)");
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let music_label = if self.audio.music_looping { "[Stop]" } else { "[Play]" };
            if ui.button(music_label).clicked() {
                self.audio.music_looping = !self.audio.music_looping;
                if let Some(tx) = &self.audio_cmd_tx {
                    if self.audio.music_looping {
                        self.audio.music_pan = 0.0;
                        let _ = tx.send(AudioCommand::StartMusic { path: "music.ogg".to_string() });
                    } else {
                        let _ = tx.send(AudioCommand::StopMusic);
                    }
                }
            }
            if self.audio.music_looping {
                ui.label("Balance :");
                let pan_slider = egui::Slider::new(&mut self.audio.music_pan, -1.0..=1.0)
                    .custom_formatter(|v, _| {
                        if v < -0.8 { "Gauche".to_string() }
                        else if v <= 0.2 { "Centre".to_string() }
                        else { "Droite".to_string() }
                    });
                let response = ui.add_sized([ui.available_width(), 20.0], pan_slider);
                if response.drag_stopped() || (response.changed() && !response.dragged()) {
                    if let Some(tx) = &self.audio_cmd_tx {
                        let _ = tx.send(AudioCommand::SetMusicPan { pan: self.audio.music_pan });
                    }
                }
            }
        });

        ui.add_space(12.0);
        ui.strong("Test 2 - Enceintes Playfield (exciters SSF)");
        ui.add_space(4.0);
        ui.label("Posez vos mains sur les coins de la caisse et testez chaque exciter :");
        ui.add_space(4.0);

        // 4 speaker buttons in a square layout + 2 ball tests in the middle
        let btn_w = 140.0;
        let btn_h = 30.0;
        let gap = 20.0;

        // Row 1: Top Left / Top Right
        ui.horizontal(|ui| {
            ui.add_space(gap);
            if ui.add_sized([btn_w, btn_h], egui::Button::new("Haut Gauche (BL)")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayOnSpeaker {
                        path: "ball_roll.ogg".to_string(), target: audio::SpeakerTarget::TopLeft,
                    });
                }
            }
            ui.add_space(gap * 2.0);
            if ui.add_sized([btn_w, btn_h], egui::Button::new("Haut Droite (BR)")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayOnSpeaker {
                        path: "ball_roll.ogg".to_string(), target: audio::SpeakerTarget::TopRight,
                    });
                }
            }
        });

        // Row 2: Ball test buttons (centered)
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(gap + btn_w / 2.0);
            if ui.add_sized([btn_w + gap, btn_h], egui::Button::new("Bille Haut > Bas")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayBallSequence {
                        path: "ball_roll.ogg".to_string(),
                        from: audio::SpeakerTarget::TopBoth,
                        to: audio::SpeakerTarget::BottomBoth,
                        hold_start_ms: 1500,
                        fade_ms: 3000,
                        hold_end_ms: 1500,
                    });
                }
            }
        });
        ui.horizontal(|ui| {
            ui.add_space(gap + btn_w / 2.0);
            if ui.add_sized([btn_w + gap, btn_h], egui::Button::new("Bille Gauche > Droite")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayBallSequence {
                        path: "ball_roll.ogg".to_string(),
                        from: audio::SpeakerTarget::LeftBoth,
                        to: audio::SpeakerTarget::RightBoth,
                        hold_start_ms: 1500,
                        fade_ms: 3000,
                        hold_end_ms: 1500,
                    });
                }
            }
        });
        ui.add_space(4.0);

        // Row 3: Bottom Left / Bottom Right
        ui.horizontal(|ui| {
            ui.add_space(gap);
            if ui.add_sized([btn_w, btn_h], egui::Button::new("Bas Gauche (SL)")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayOnSpeaker {
                        path: "ball_roll.ogg".to_string(), target: audio::SpeakerTarget::BottomLeft,
                    });
                }
            }
            ui.add_space(gap * 2.0);
            if ui.add_sized([btn_w, btn_h], egui::Button::new("Bas Droite (SR)")).clicked() {
                if let Some(tx) = &self.audio_cmd_tx {
                    let _ = tx.send(AudioCommand::PlayOnSpeaker {
                        path: "ball_roll.ogg".to_string(), target: audio::SpeakerTarget::BottomRight,
                    });
                }
            }
        });
    }

    fn render_tables_dir_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Repertoire des tables");
        ui.add_space(4.0);
        ui.label("Selectionnez le repertoire contenant vos tables VPX (un dossier par table).");
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("Chemin :");
            ui.text_edit_singleline(&mut self.tables_dir);
            if ui.button("Parcourir...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Selectionnez le repertoire des tables")
                    .pick_folder()
                {
                    self.tables_dir = path.to_string_lossy().into_owned();
                }
            }
        });

        if !self.tables_dir.is_empty() {
            let path = std::path::Path::new(&self.tables_dir);
            if path.is_dir() {
                // Count table folders
                let count = std::fs::read_dir(path)
                    .map(|entries| {
                        entries
                            .filter_map(|e| e.ok())
                            .filter(|e| e.path().is_dir())
                            .count()
                    })
                    .unwrap_or(0);
                ui.add_space(8.0);
                ui.label(format!("Repertoire valide - {count} dossiers trouves"));
            } else {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::RED, "Ce repertoire n'existe pas");
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Check quit timer (after knocker plays)
        if let Some(start) = self.quit_after_ms {
            if start.elapsed().as_millis() > 800 {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }
            ui.ctx().request_repaint(); // keep ticking
        }

        // Process joystick events (shared between tilt viz and input capture)
        if let Some(rx) = &self.joystick_rx {
            while let Ok(event) = rx.try_recv() {
                match &event {
                    JoystickEvent::AccelUpdate { x, y } => {
                        self.accel_x = *x;
                        self.accel_y = *y;
                    }
                    JoystickEvent::ButtonDown { device_id, button, name } => {
                        // If capturing input, assign it
                        if let CaptureState::Capturing(idx) = self.capture_state {
                            if idx < self.actions.len() {
                                self.actions[idx].mapping = Some(CapturedInput::JoystickButton {
                                    device_id: device_id.clone(),
                                    button: *button,
                                    name: name.clone(),
                                });
                            }
                            self.capture_state = CaptureState::Idle;
                        }
                    }
                    JoystickEvent::AxisMotion { device_id, axis, name } => {
                        if let CaptureState::Capturing(idx) = self.capture_state {
                            if idx < self.actions.len() {
                                self.actions[idx].mapping = Some(CapturedInput::JoystickAxis {
                                    device_id: device_id.clone(),
                                    axis: *axis,
                                    name: name.clone(),
                                });
                            }
                            self.capture_state = CaptureState::Idle;
                        }
                    }
                }
            }
        }

        // Header
        egui::Panel::top("wizard_header").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("PinReady");
                ui.separator();
                for i in 0..WizardPage::count() {
                    let page = WizardPage::from_index(i).unwrap();
                    let is_current = page == self.page;
                    let label = format!("{}. {}", i + 1, page.title());
                    if is_current {
                        ui.strong(&label);
                    } else {
                        ui.label(&label);
                    }
                    if i < WizardPage::count() - 1 {
                        ui.label(">");
                    }
                }
            });
        });

        // Navigation footer
        egui::Panel::bottom("wizard_nav").show_inside(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if self.page.index() > 0 {
                    if ui.button("< Precedent").clicked() {
                        self.prev_page();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.page.index() < WizardPage::count() - 1 {
                        if ui.button("Suivant >").clicked() {
                            self.next_page();
                        }
                    } else if ui.button("Terminer").clicked() {
                        self.finalize_wizard(ui.ctx());
                    }
                });
            });
            ui.add_space(4.0);
        });

        // Main content
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.page {
                    WizardPage::Screens => self.render_screens_page(ui),
                    WizardPage::Rendering => self.render_rendering_page(ui),
                    WizardPage::Inputs => self.render_inputs_page(ui),
                    WizardPage::Tilt => self.render_tilt_page(ui),
                    WizardPage::Audio => self.render_audio_page(ui),
                    WizardPage::TablesDir => self.render_tables_dir_page(ui),
                }
            });
        });
    }
}
