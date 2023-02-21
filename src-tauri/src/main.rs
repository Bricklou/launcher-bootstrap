#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod deep_link;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn init() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // initialize the deep link handler
    deep_link::prepare("ovh.bricklou.launcher-bootstrap");
    // It's expected to use the identifier from tauri.conf.json
    // Unfortuenetly getting it is pretty ugly without access to sth that implements `Manager`.
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // This could be called right after prepare() but then you don't have access to the tauri APIs
    let handle = app.handle();
    deep_link::register("launcher-bootstrap", move |request| {
        dbg!(&request);
        handle.emit_all("scheme-request-received", request).unwrap();
    });
    Ok(())
}

fn main() {
    init();

    info!("Launcher bootstrap version {}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
