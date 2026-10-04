use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation, glib};

fn main() {
    let app = Application::builder().application_id("com.my.app").build();
    app.connect_activate(|app| {
        let windows = ApplicationWindow::new(app);
        windows.set_default_size(500, 500);
        windows.set_title(Some("Gtk Windows"));

        let button = Button::new();
        button.set_label("Count");
        button.connect_clicked(|_button| {});

        windows.set_child(Some(&button));

        windows.show();
    });

    app.run();
}
