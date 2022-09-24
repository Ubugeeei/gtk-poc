use gtk::{gdk::Display, prelude::*, Application, CssProvider, StyleContext};

pub fn build_ui(app: &Application, _files: &[gtk::gio::File], _blah: &str) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1280)
        .height_request(800)
        .build();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    container.set_margin_top(10);

    window.set_child(Some(&container));

    let text = gtk::Label::builder()
        .label("Hello World!")
        .css_classes(vec!["text".to_string()])
        .build();

    container.append(&text);

    window.present();
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(
        r#"
            .text {
                font-size: 30px;
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
