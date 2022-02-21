use bevy::prelude::*;
use std::thread;

mod app;
use app::App as MyApp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup() {
    thread::spawn(|| {
        let mut app = MyApp::new();
        loop {
            app.update();
        }
    });
}
