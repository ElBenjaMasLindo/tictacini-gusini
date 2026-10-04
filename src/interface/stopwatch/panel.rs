use super::timer as T;

use adw::prelude::*; // GTK
use adw::gtk::{Box, Button, Label, Orientation};

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
    

    pagina.append(&clock(&mut timer));
    pagina.append(&switch(&mut timer));
    
    pagina
}

fn clock(_timer: &mut T::Timer) -> Label {
    let clock = Label::new(Some("00:00"));
    clock
}

fn switch(_timer: &mut T::Timer) -> Button {
    let button = Button::new();
    button
}