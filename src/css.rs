use gtk::gdk::Display;
use gtk::{prelude::*, Button, CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application, _files: &[gtk::gio::File], _blah: &str) {
    // Create button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();
    window.show();
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(
        r#"
          button {
            color: magenta;
          }
    "#
        .as_bytes(),
    );

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
