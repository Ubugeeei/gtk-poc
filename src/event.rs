use gtk::{prelude::*, Button};
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application, _files: &[gtk::gio::File], _blah: &str) {
    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        // GUI is blocked for 5 seconds after the button is pressed
        println!("Button clicked!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();
}
