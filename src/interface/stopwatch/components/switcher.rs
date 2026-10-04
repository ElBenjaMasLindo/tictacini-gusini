use super::super::timer as T;
use crate::G;

use adw::prelude::*; // GTK
use adw::gtk::{Button};

use std::cell::RefCell;
use std::rc::Rc;

fn state_to_text(state: &T::States) -> String {
    match state {
        T::States::Init => return "Arrancar".to_string(),
        T::States::Other => return "Descansando".to_string(),
        T::States::Work => return "Trabajando".to_string()
    }
}

pub fn index(timer: Rc<RefCell<T::Timer>>) -> Button {
    let button = Button::new();
    
    let timer = timer.clone();

    button.set_size_request(200, 200);
    G::space(&button);
    button.set_label(&state_to_text(&timer.borrow().state));

    button.connect_clicked(move |button| {
        timer.borrow_mut().play();
        button.set_label(&state_to_text(&timer.borrow().state));
    });

    button
}