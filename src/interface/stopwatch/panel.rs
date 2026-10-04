use super::timer as T;

use adw::prelude::*; // GTK
use adw::gtk::{Box, Button, Label, Orientation, glib};

use crate::global;

fn state_to_text(state: T::States) -> String {
    match state {
        T::States::Init => return "Arrancar".to_string(),
        T::States::Other => return "Descansando".to_string(),
        T::States::Work => return "Trabajando".to_string()
    }
}

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, global::PADDING);

    let mut timer = T::Timer::new();

    let estado = Label::new(Some("Laburando"));
    let clock = Label::new(Some("00:00"));
    
    let switch = Button::new();
    switch.set_label("Arrancar");

    pagina.append(&estado);
    pagina.append(&clock);
    pagina.append(&switch);
    
    pagina
}