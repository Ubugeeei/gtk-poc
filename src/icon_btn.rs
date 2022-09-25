use gtk::gdk::Display;
use gtk::{prelude::*, Button, CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application, _files: &[gtk::gio::File], _blah: &str) {
    // Create a button with label and margins
    let button = Button::builder()
        // NOTE: http://standards.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
        .icon_name("view-refresh")
        .css_classes(vec!["refresh-icon".to_string()])
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |btn| {
        let i_name = btn.icon_name();
        if i_name.unwrap() == "view-refresh" {
            btn.set_icon_name("window-close");
        } else {
            btn.set_icon_name("view-refresh");
        }
    });

    // Create a windowC
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(
        r#"
      .refresh-icon {
        border: none;
        box-shadow: none;
        cursor: pointer;
        border-radius: 50%;
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
