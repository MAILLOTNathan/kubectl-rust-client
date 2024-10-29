use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use serde_json::Value;

mod utils;
mod env;

fn create_app(name: &str) -> Application {
    let app: Application = Application::builder()
        .application_id(name)
        .build();
    return app;
}

fn create_window(app: &Application, specs: &Value) -> ApplicationWindow {
    let win: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .default_width(specs["width"].as_i64().unwrap() as i32)
        .default_height(specs["height"].as_i64().unwrap() as i32)
        .title(specs["title"].to_string())
        .build();
    return win;
}

fn main() {
    let app: Application = create_app(env::config::APP_NAME);

    app.connect_activate(|app: &Application| {
        let win: ApplicationWindow = create_window(app, &utils::myjson::parse_file(env::config::WINDOW_CONFIG_FILE).unwrap());

        win.show_all();
    });

    app.run();
}