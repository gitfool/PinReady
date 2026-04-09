// PinReady — VPinballX configuration wizard & table launcher
// Copyright (C) 2026 — Licensed under GPLv3+
// See https://www.gnu.org/licenses/gpl-3.0.html

mod app;
mod audio;
mod config;
mod db;
mod inputs;
mod screens;
mod tilt;

use anyhow::Result;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("PinReady v{VERSION}");
        println!("License: GPLv3+ — https://www.gnu.org/licenses/gpl-3.0.html");
        return Ok(());
    }
    let force_config = args.iter().any(|a| a == "--config" || a == "-c");
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("PinReady v{VERSION} — VPinballX configurator & launcher");
        println!();
        println!("Usage: pinready [OPTIONS]");
        println!();
        println!("Options:");
        println!("  --config, -c    Force configuration wizard mode");
        println!("  --version, -v   Show version and license");
        println!("  --help, -h      Show this help");
        return Ok(());
    }

    env_logger::init();
    log::info!("PinReady v{VERSION} starting...");

    // Initialize SDL3 for display enumeration only (joystick + audio handled in their threads)
    unsafe {
        use sdl3_sys::everything::*;
        if !SDL_Init(SDL_INIT_VIDEO) {
            let err = std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy();
            anyhow::bail!("Failed to init SDL3: {err}");
        }
    }
    log::info!("SDL3 initialized (video for display enumeration)");

    // Open database
    let db = db::Database::open(None)?;
    let is_configured = db.is_configured();

    // Load VPX config (pre-fill wizard if ini exists)
    let vpx_config = config::VpxConfig::load(None)?;

    // Determine start mode:
    // - --config flag → wizard
    // - No DB or no VPX ini → wizard (first run)
    // - Otherwise → launcher
    let start_in_wizard = force_config || !is_configured;
    if start_in_wizard {
        log::info!("Starting in configuration wizard mode");
    } else {
        log::info!("Starting in launcher mode");
    }

    // Create app (starts joystick + audio threads internally)
    let app = app::App::new(vpx_config, db, start_in_wizard);

    // Launch eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("PinReady v{VERSION}"))
            .with_inner_size([1000.0, 1000.0]),
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
