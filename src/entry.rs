use gtk::prelude::*;

pub fn build_ui(app: &gtk::Application, _files: &[gtk::gio::File], _blah: &str) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("gtk input")
        .width_request(1280)
        .height_request(800)
        .build();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&container));

    let input_field = gtk::Entry::builder()
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // handle the input
    input_field.connect_changed(move |input_field| {
        println!("Input field changed: {}", input_field.text());
    });

    container.append(&input_field);

    window.present();
}
