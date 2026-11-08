mod steam_scanner;
mod steam_cache;
mod steam_api;
mod injector;
mod ui_main;
mod utils;
mod steam_vdf;
mod smokeapi_manager;
mod config;


use libadwaita::prelude::*;
use tokio::runtime::Runtime;

fn main() -> gtk4::glib::ExitCode {
    env_logger::init();

    // Initialize Tokio runtime in a separate thread if needed, or just use it for spawn_blocking
    // Since we're using glib::spawn_future_local, we need a way to run tokio tasks.
    let _rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = _rt.enter();

    let app = libadwaita::Application::builder()
        .application_id("io.github.vlapochkin.vapordose")
        .build();

    app.connect_activate(ui_main::build_ui);

    app.run()
}
