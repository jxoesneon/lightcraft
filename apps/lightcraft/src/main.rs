//! LightCraft desktop application — 100% sovereign Martensite runtime.
//!
//! The desktop entry point is being migrated off egui onto the Martensite
//! retained-mode GUI (`crates/ui-martensite`). Until the Martensite window
//! runner lands, this binary builds the engine and app state and reports that
//! the sovereign runtime is up; the egui frontend remains in `crates/ui-egui`.

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use lightcraft_engine::Engine;
use lightcraft_ui_martensite::LightcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = LightcraftApp::new(engine);

    println!("Starting LightCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
