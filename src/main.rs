use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::Application;

mod button;
mod window;

const SAMPLE_PJ_NAME_HELP: &str = r#"
SAMPLE_PJ_NAME:
    - window
    - button
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: cargo run [SAMPLE_PJ_NAME]");
        println!("{}", SAMPLE_PJ_NAME_HELP);
        return;
    }

    let build_ui = match &*args[1] {
        "window" => window::build_ui,
        "button" => button::build_ui,
        _ => {
            println!("Unknown sample_pj_name: {}", &args[1]);
            println!("{}", SAMPLE_PJ_NAME_HELP);
            panic!()
        }
    };

    let app = Application::new(Some("com.example.App"), ApplicationFlags::HANDLES_OPEN);

    // Connect to "activate" signal of `app`
    // app.connect_activate(build_ui);
    app.connect_open(build_ui);

    // Run the application
    // app.run_with_args(&args);
    app.run();
}
