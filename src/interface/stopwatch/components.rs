use super::timer as T;
use crate::G;

use adw::prelude::*; // GTK
use adw::gtk::{Button, Label, glib};

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn state_to_text(state: &T::States) -> String {
    match state {
        T::States::Init => return "Arrancar".to_string(),
        T::States::Other => return "Descansando".to_string(),
        T::States::Work => return "Trabajando".to_string()
    }
}

pub fn clock(_timer: &mut T::Timer) -> Label {
    let clock = Label::new(Some("00:00"));
    clock.set_size_request(500, 500);
    G::space(&clock);
    
    clock
}

pub fn switch(timer: &mut T::Timer) -> Button {
    let button = Button::new();

    button.set_size_request(200, 200);
    G::space(&button);

    button
}