use sdl3_sys::everything::*;
use std::ffi::CStr;
use std::thread;

/// An input action that can be mapped to a key or button.
#[derive(Clone)]
pub struct InputAction {
    pub setting_id: &'static str,
    pub label: &'static str,
    pub default_scancode: SDL_Scancode,
    pub essential: bool,
    pub mapping: Option<CapturedInput>,
}

/// A captured input event (keyboard or joystick).
#[derive(Clone)]
pub enum CapturedInput {
    Keyboard {
        scancode: SDL_Scancode,
        name: String,
    },
    JoystickButton {
        device_id: String,
        button: u8,
        name: String,
    },
    #[allow(dead_code)]
    JoystickAxis {
        device_id: String,
        axis: u8,
        name: String,
    },
}

impl CapturedInput {
    /// Format as VPX ini mapping string.
    pub fn to_mapping_string(&self) -> String {
        match self {
            Self::Keyboard { scancode, .. } => {
                format!("Key;{}", scancode.0)
            }
            Self::JoystickButton { device_id, button, .. } => {
                format!("{device_id};{button}")
            }
            Self::JoystickAxis { device_id, axis, .. } => {
                format!("{device_id};{axis}")
            }
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Keyboard { name, .. } => name,
            Self::JoystickButton { name, .. } => name,
            Self::JoystickAxis { name, .. } => name,
        }
    }
}

/// Joystick events sent from the SDL3 joystick thread to the UI.
#[derive(Clone)]
pub enum JoystickEvent {
    ButtonDown { device_id: String, button: u8, name: String },
    AxisMotion { device_id: String, axis: u8, name: String },
    /// Live accelerometer/axis data for visualization (axis_id, normalized value -1.0 to 1.0)
    AccelUpdate { x: f32, y: f32 },
}

/// Get a human-readable name for an SDL scancode.
pub fn scancode_name(scancode: SDL_Scancode) -> String {
    unsafe {
        let name_ptr = SDL_GetScancodeName(scancode);
        if !name_ptr.is_null() {
            let s = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
            if !s.is_empty() {
                return s;
            }
        }
    }
    format!("Key {}", scancode.0)
}

/// Convert an egui::Key to an SDL_Scancode.
/// This allows capturing keyboard input via egui (which has the window focus)
/// instead of a separate SDL3 thread.
pub fn egui_key_to_scancode(key: egui::Key) -> Option<SDL_Scancode> {
    use egui::Key;
    let sc = match key {
        Key::A => SDL_SCANCODE_A,
        Key::B => SDL_SCANCODE_B,
        Key::C => SDL_SCANCODE_C,
        Key::D => SDL_SCANCODE_D,
        Key::E => SDL_SCANCODE_E,
        Key::F => SDL_SCANCODE_F,
        Key::G => SDL_SCANCODE_G,
        Key::H => SDL_SCANCODE_H,
        Key::I => SDL_SCANCODE_I,
        Key::J => SDL_SCANCODE_J,
        Key::K => SDL_SCANCODE_K,
        Key::L => SDL_SCANCODE_L,
        Key::M => SDL_SCANCODE_M,
        Key::N => SDL_SCANCODE_N,
        Key::O => SDL_SCANCODE_O,
        Key::P => SDL_SCANCODE_P,
        Key::Q => SDL_SCANCODE_Q,
        Key::R => SDL_SCANCODE_R,
        Key::S => SDL_SCANCODE_S,
        Key::T => SDL_SCANCODE_T,
        Key::U => SDL_SCANCODE_U,
        Key::V => SDL_SCANCODE_V,
        Key::W => SDL_SCANCODE_W,
        Key::X => SDL_SCANCODE_X,
        Key::Y => SDL_SCANCODE_Y,
        Key::Z => SDL_SCANCODE_Z,
        Key::Num0 => SDL_SCANCODE_0,
        Key::Num1 => SDL_SCANCODE_1,
        Key::Num2 => SDL_SCANCODE_2,
        Key::Num3 => SDL_SCANCODE_3,
        Key::Num4 => SDL_SCANCODE_4,
        Key::Num5 => SDL_SCANCODE_5,
        Key::Num6 => SDL_SCANCODE_6,
        Key::Num7 => SDL_SCANCODE_7,
        Key::Num8 => SDL_SCANCODE_8,
        Key::Num9 => SDL_SCANCODE_9,
        Key::Escape => SDL_SCANCODE_ESCAPE,
        Key::Tab => SDL_SCANCODE_TAB,
        Key::Space => SDL_SCANCODE_SPACE,
        Key::Enter => SDL_SCANCODE_RETURN,
        Key::Backspace => SDL_SCANCODE_BACKSPACE,
        Key::Delete => SDL_SCANCODE_DELETE,
        Key::Home => SDL_SCANCODE_HOME,
        Key::End => SDL_SCANCODE_END,
        Key::PageUp => SDL_SCANCODE_PAGEUP,
        Key::PageDown => SDL_SCANCODE_PAGEDOWN,
        Key::ArrowUp => SDL_SCANCODE_UP,
        Key::ArrowDown => SDL_SCANCODE_DOWN,
        Key::ArrowLeft => SDL_SCANCODE_LEFT,
        Key::ArrowRight => SDL_SCANCODE_RIGHT,
        Key::F1 => SDL_SCANCODE_F1,
        Key::F2 => SDL_SCANCODE_F2,
        Key::F3 => SDL_SCANCODE_F3,
        Key::F4 => SDL_SCANCODE_F4,
        Key::F5 => SDL_SCANCODE_F5,
        Key::F6 => SDL_SCANCODE_F6,
        Key::F7 => SDL_SCANCODE_F7,
        Key::F8 => SDL_SCANCODE_F8,
        Key::F9 => SDL_SCANCODE_F9,
        Key::F10 => SDL_SCANCODE_F10,
        Key::F11 => SDL_SCANCODE_F11,
        Key::F12 => SDL_SCANCODE_F12,
        Key::Minus => SDL_SCANCODE_MINUS,
        Key::Comma => SDL_SCANCODE_COMMA,
        Key::Period => SDL_SCANCODE_PERIOD,
        _ => return None,
    };
    Some(sc)
}

/// Check for modifier keys pressed in egui and return matching scancode.
pub fn egui_modifiers_to_scancode(modifiers: &egui::Modifiers) -> Option<SDL_Scancode> {
    if modifiers.shift && !modifiers.ctrl && !modifiers.alt {
        Some(SDL_SCANCODE_LSHIFT)
    } else if modifiers.ctrl && !modifiers.shift && !modifiers.alt {
        Some(SDL_SCANCODE_LCTRL)
    } else if modifiers.alt && !modifiers.shift && !modifiers.ctrl {
        Some(SDL_SCANCODE_LALT)
    } else {
        None
    }
}

/// Build the list of all mappable actions with their defaults.
pub fn default_actions() -> Vec<InputAction> {
    vec![
        // Essential — ordered: flippers, staged, magna, then game controls
        InputAction { setting_id: "LeftFlipper", label: "Left Flipper", default_scancode: SDL_SCANCODE_LSHIFT, essential: true, mapping: None },
        InputAction { setting_id: "RightFlipper", label: "Right Flipper", default_scancode: SDL_SCANCODE_RSHIFT, essential: true, mapping: None },
        InputAction { setting_id: "LeftStagedFlipper", label: "Left Staged Flipper", default_scancode: SDL_SCANCODE_LSHIFT, essential: true, mapping: None },
        InputAction { setting_id: "RightStagedFlipper", label: "Right Staged Flipper", default_scancode: SDL_SCANCODE_RSHIFT, essential: true, mapping: None },
        InputAction { setting_id: "LeftMagna", label: "Left Magna", default_scancode: SDL_SCANCODE_LCTRL, essential: true, mapping: None },
        InputAction { setting_id: "RightMagna", label: "Right Magna", default_scancode: SDL_SCANCODE_RCTRL, essential: true, mapping: None },
        InputAction { setting_id: "Lockbar", label: "Lockbar", default_scancode: SDL_SCANCODE_LALT, essential: true, mapping: None },
        InputAction { setting_id: "ExtraBall", label: "Extra Ball", default_scancode: SDL_SCANCODE_B, essential: true, mapping: None },
        InputAction { setting_id: "LaunchBall", label: "Launch Ball", default_scancode: SDL_SCANCODE_RETURN, essential: true, mapping: None },
        InputAction { setting_id: "Start", label: "Start Game", default_scancode: SDL_SCANCODE_1, essential: true, mapping: None },
        InputAction { setting_id: "Credit1", label: "Add Credit", default_scancode: SDL_SCANCODE_5, essential: true, mapping: None },
        InputAction { setting_id: "ExitGame", label: "Exit Game", default_scancode: SDL_SCANCODE_ESCAPE, essential: true, mapping: None },
        // Advanced — all remaining mappable actions
        InputAction { setting_id: "Credit2", label: "Add Credit (2)", default_scancode: SDL_SCANCODE_4, essential: false, mapping: None },
        InputAction { setting_id: "Credit3", label: "Add Credit (3)", default_scancode: SDL_SCANCODE_3, essential: false, mapping: None },
        InputAction { setting_id: "Credit4", label: "Add Credit (4)", default_scancode: SDL_SCANCODE_6, essential: false, mapping: None },
        InputAction { setting_id: "CoinDoor", label: "Coin Door", default_scancode: SDL_SCANCODE_END, essential: false, mapping: None },
        InputAction { setting_id: "SlamTilt", label: "Slam Tilt", default_scancode: SDL_SCANCODE_HOME, essential: false, mapping: None },
        InputAction { setting_id: "Reset", label: "Reset Game", default_scancode: SDL_SCANCODE_F3, essential: false, mapping: None },
        InputAction { setting_id: "Service1", label: "Service #1", default_scancode: SDL_SCANCODE_7, essential: false, mapping: None },
        InputAction { setting_id: "Service2", label: "Service #2", default_scancode: SDL_SCANCODE_8, essential: false, mapping: None },
        InputAction { setting_id: "Service3", label: "Service #3", default_scancode: SDL_SCANCODE_9, essential: false, mapping: None },
        InputAction { setting_id: "Service4", label: "Service #4", default_scancode: SDL_SCANCODE_0, essential: false, mapping: None },
        InputAction { setting_id: "Service5", label: "Service #5", default_scancode: SDL_SCANCODE_6, essential: false, mapping: None },
        InputAction { setting_id: "Service6", label: "Service #6", default_scancode: SDL_SCANCODE_PAGEUP, essential: false, mapping: None },
        InputAction { setting_id: "Service7", label: "Service #7", default_scancode: SDL_SCANCODE_MINUS, essential: false, mapping: None },
        InputAction { setting_id: "Service8", label: "Service #8", default_scancode: SDL_SCANCODE_UNKNOWN, essential: false, mapping: None },
        InputAction { setting_id: "LeftNudge", label: "Left Nudge", default_scancode: SDL_SCANCODE_Z, essential: false, mapping: None },
        InputAction { setting_id: "RightNudge", label: "Right Nudge", default_scancode: SDL_SCANCODE_SLASH, essential: false, mapping: None },
        InputAction { setting_id: "CenterNudge", label: "Center Nudge", default_scancode: SDL_SCANCODE_SPACE, essential: false, mapping: None },
        InputAction { setting_id: "Tilt", label: "Tilt", default_scancode: SDL_SCANCODE_T, essential: false, mapping: None },
        InputAction { setting_id: "Pause", label: "Pause", default_scancode: SDL_SCANCODE_P, essential: false, mapping: None },
        InputAction { setting_id: "VolumeDown", label: "Volume Down", default_scancode: SDL_SCANCODE_MINUS, essential: false, mapping: None },
        InputAction { setting_id: "VolumeUp", label: "Volume Up", default_scancode: SDL_SCANCODE_EQUALS, essential: false, mapping: None },
        InputAction { setting_id: "Custom1", label: "Custom #1", default_scancode: SDL_SCANCODE_UNKNOWN, essential: false, mapping: None },
        InputAction { setting_id: "Custom2", label: "Custom #2", default_scancode: SDL_SCANCODE_UNKNOWN, essential: false, mapping: None },
        InputAction { setting_id: "Custom3", label: "Custom #3", default_scancode: SDL_SCANCODE_UNKNOWN, essential: false, mapping: None },
        InputAction { setting_id: "Custom4", label: "Custom #4", default_scancode: SDL_SCANCODE_UNKNOWN, essential: false, mapping: None },
    ]
}

/// Check if any two actions share the same key binding.
pub fn find_conflicts(actions: &[InputAction]) -> Vec<(usize, usize)> {
    let mut conflicts = Vec::new();
    for i in 0..actions.len() {
        for j in (i + 1)..actions.len() {
            let a = effective_scancode(&actions[i]);
            let b = effective_scancode(&actions[j]);
            if a != SDL_SCANCODE_UNKNOWN && a == b {
                conflicts.push((i, j));
            }
        }
    }
    conflicts
}

fn effective_scancode(action: &InputAction) -> SDL_Scancode {
    match &action.mapping {
        Some(CapturedInput::Keyboard { scancode, .. }) => *scancode,
        None => action.default_scancode,
        _ => SDL_SCANCODE_UNKNOWN,
    }
}

/// Spawn the SDL3 joystick-only thread (keyboard is handled via egui).
/// Returns a receiver for joystick events.
pub fn spawn_joystick_thread() -> crossbeam_channel::Receiver<JoystickEvent> {
    let (evt_tx, evt_rx) = crossbeam_channel::unbounded::<JoystickEvent>();

    thread::spawn(move || {
        unsafe {
            // Init joystick subsystem in this thread
            if !SDL_InitSubSystem(SDL_INIT_JOYSTICK) {
                log::error!("Joystick thread: SDL_InitSubSystem failed: {:?}", CStr::from_ptr(SDL_GetError()));
                return;
            }

            // Open all connected joysticks
            let mut joy_count: i32 = 0;
            let joy_ids = SDL_GetJoysticks(&mut joy_count);
            if !joy_ids.is_null() {
                for i in 0..joy_count as usize {
                    let jid = *joy_ids.add(i);
                    let joy = SDL_OpenJoystick(jid);
                    if !joy.is_null() {
                        let name_ptr = SDL_GetJoystickName(joy);
                        let name = if !name_ptr.is_null() {
                            CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
                        } else {
                            format!("Joystick {}", jid.0)
                        };
                        log::info!("Opened joystick: {} (id={})", name, jid.0);
                    }
                }
                SDL_free(joy_ids as *mut _);
            }
            log::info!("Joystick thread started, {} joystick(s) found", joy_count);

            let mut accel_x: f32 = 0.0;
            let mut accel_y: f32 = 0.0;

            loop {
                let mut event: SDL_Event = std::mem::zeroed();
                while SDL_PollEvent(&mut event) {
                    match SDL_EventType(event.r#type) {
                        SDL_EVENT_JOYSTICK_BUTTON_DOWN => {
                            let jbutton = event.jbutton;
                            let device_id = format!("Joy{}", jbutton.which.0);
                            let name = format!("Joy{} Button {}", jbutton.which.0, jbutton.button);
                            let _ = evt_tx.send(JoystickEvent::ButtonDown {
                                device_id,
                                button: jbutton.button,
                                name,
                            });
                        }
                        SDL_EVENT_JOYSTICK_AXIS_MOTION => {
                            let jaxis = event.jaxis;
                            // Send axis for input capture (only on big movement)
                            if jaxis.value.abs() > 16000 {
                                let device_id = format!("Joy{}", jaxis.which.0);
                                let name = format!("Joy{} Axis {}", jaxis.which.0, jaxis.axis);
                                let _ = evt_tx.send(JoystickEvent::AxisMotion {
                                    device_id,
                                    axis: jaxis.axis,
                                    name,
                                });
                            }
                            // Send live accel data for tilt visualization
                            // Axes 0,1 are typically X,Y accelerometer on KL25Z/Pinscape
                            let norm = jaxis.value as f32 / 32767.0;
                            if jaxis.axis == 0 {
                                accel_x = norm;
                            } else if jaxis.axis == 1 {
                                accel_y = norm;
                            }
                            let _ = evt_tx.send(JoystickEvent::AccelUpdate { x: accel_x, y: accel_y });
                        }
                        SDL_EVENT_JOYSTICK_ADDED => {
                            let jdevice = event.jdevice;
                            let joy = SDL_OpenJoystick(jdevice.which);
                            if !joy.is_null() {
                                let name_ptr = SDL_GetJoystickName(joy);
                                let name = if !name_ptr.is_null() {
                                    CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
                                } else {
                                    format!("Joystick {}", jdevice.which.0)
                                };
                                log::info!("Joystick connected: {}", name);
                            }
                        }
                        SDL_EVENT_QUIT => return,
                        _ => {}
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    });

    evt_rx
}
