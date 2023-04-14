mod events;
mod paths;

use tauri::Manager;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{constants, deep_link};

mod commands;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    // register the subscriber as the global default for this application.
    tracing::subscriber::set_global_default(subscriber)?;

    // initialize the deep link handler
    deep_link::prepare(constants::IDENTIFIER);
    // It's expected to use the identifier from tauri.conf.json
    // Unfortunately getting it is pretty ugly without access to sth that implements `Manager`.

    Ok(())
}

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // This could be called right after prepare() but then you don't have access to the Tauri APIs.
    let handle = app.handle();

    deep_link::register("launcher-bootstrap", move |request| {
        dbg!(&request);
        events::register_link_events(request, &handle).unwrap();
    })?;

    #[cfg(debug_assertions)]
    {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::get_or_fetch_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
