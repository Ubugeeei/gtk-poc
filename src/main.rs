use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;

mod button;
mod css;
mod entry;
mod event;
mod window;

const SAMPLE_PJ_NAME_HELP: &str = r#"
SAMPLE_PJ_NAME:
    - window
    - button
    - event
    - css
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
            app.connect_open(entry::build_ui);
            app
        }
        _ => {
            println!("Unknown sample_pj_name: {}", &args[1]);
            println!("{}", SAMPLE_PJ_NAME_HELP);
            panic!()
        }
    };

    // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);

    // Run the application
    // app.run_with_args(&args);
    app.run();
}
