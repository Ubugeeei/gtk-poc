use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application, _files: &[gtk::gio::File], _blah: &str) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();
    // Present window
    window.present();
}
