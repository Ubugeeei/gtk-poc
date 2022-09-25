use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;

mod button;
mod css;
mod entry;
mod event;
mod text;
mod window;
mod icon_btn;

const SAMPLE_PJ_NAME_HELP: &str = r#"
SAMPLE_PJ_NAME:
    - window
    - text
    - button
    - icon_btn
    - entry
    - css
    - event
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: cargo run [SAMPLE_PJ_NAME]");
        println!("{}", SAMPLE_PJ_NAME_HELP);
        return;
    }

    let app = match &*args[1] {
        "window" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_open(window::build_ui);
            app
        }
        "button" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_open(button::build_ui);
            app
        }
        "icon_btn" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_startup(|_| icon_btn::load_css());
            app.connect_open(icon_btn::build_ui);
            app
        }
        "event" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_open(event::build_ui);
            app
        }
        "css" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_startup(|_| css::load_css());
            app.connect_open(css::build_ui);
            app
        }
        "entry" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_startup(|_| entry::load_css());
            app.connect_open(entry::build_ui);
            app
        }
        "text" => {
            let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);
            app.connect_startup(|_| text::load_css());
            app.connect_open(text::build_ui);
            app
        }
        _ => {
            println!("Unknown sample_pj_name: {}", &args[1]);
            println!("{}", SAMPLE_PJ_NAME_HELP);
            panic!()
        }
    };

    app.run();
}
