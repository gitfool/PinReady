mod app;
mod audio;
mod config;
mod db;
mod inputs;
mod screens;
mod tilt;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("PinReady starting...");

    // Initialize SDL3 for display enumeration only (joystick + audio handled in their threads)
    // eframe/winit manages its own window, so SDL_INIT_VIDEO is only needed for display queries
    unsafe {
        use sdl3_sys::everything::*;
        if !SDL_Init(SDL_INIT_VIDEO) {
            let err = std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy();
            anyhow::bail!("Failed to init SDL3: {err}");
        }
    }
    log::info!("SDL3 initialized (video for display enumeration)");

    // Open database (first-run detection)
    let db = db::Database::open(None)?;
    let is_first_run = !db.is_configured();

    if is_first_run {
        log::info!("First run detected — launching configuration wizard");
    } else {
        log::info!("Configuration found — launching wizard (re-configuration mode)");
    }

    // Load VPX config (pre-fill wizard if ini exists)
    let vpx_config = config::VpxConfig::load(None)?;

    // Create app (starts joystick + audio threads internally)
    let app = app::App::new(vpx_config, db);

    // Launch eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("PinReady — VPinballX Configurator")
            .with_inner_size([960.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PinReady",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {e}"))?;

    Ok(())
}
