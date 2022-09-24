use gtk::prelude::*;

pub fn build_ui(app: &gtk::Application, _files: &[gtk::gio::File], _blah: &str) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1600)
        .height_request(1000)
        .build();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&container));

    let input_field = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .css_classes(vec!["input".to_string()])
        .build();

    // handle the input
    input_field.connect_changed(move |input_field| {
        println!("Input field changed: {}", input_field.text());
    });

    container.append(&input_field);

    window.present();
}

pub fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        r#"
            .input {
                border-radius: 50px;
            }
    "#
        .as_bytes(),
    );

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
