use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::{Application, ApplicationWindow, Notebook, Box, Button, Label, Orientation, glib};

fn index(app: &Application) {
    let index = ApplicationWindow::builder()
        .application(app)
        .title("Tictacini Gusini")
        .build();

    index.present();
}

fn main() {
    let app = Application::builder().application_id("com.my.app").build();
    app.connect_activate(move |app| {
        let espacio = 15;
        
        let window = ApplicationWindow::new(app);
        window.set_default_size(500, 500);
        window.set_title(Some("Gtk Window"));

        let index = gtk::Box::new(gtk::Orientation::Vertical, espacio);

        let barrita = gtk::Box::new(gtk::Orientation::Horizontal, espacio);
        index.append(&barrita);

        
        let cronometro = gtk::Button::new();
        barrita.append(&cronometro);

        cronometro.set_halign(gtk::Align::Center);
        cronometro.set_label("Cronometro");
        cronometro.connect_clicked(|cronometro| {
            let contador = cronometro.label().unwrap_or_default().parse::<u32>().unwrap_or(0) + 1;
            cronometro.set_label(&contador.to_string());
        });

        window.set_child(Some(&index));

        window.present();
    });

    app.run();
}
