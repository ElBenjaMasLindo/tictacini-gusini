use super::super::timer as T;
use super::shared;
use crate::gb;

use adw::prelude::*; // GTK
use adw::gtk::Button;

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(timer: Rc<RefCell<T::Timer>>) -> Button {
    let button = Button::new();
    button.add_css_class("suggested-action");
    button.set_halign(gtk::Align::Center);
    
    let timer = timer.clone();

    gb::space(&button);
    button.set_label(&shared::state_to_text(&timer.borrow().state));

    button.connect_clicked(move |button| {
        timer.borrow_mut().play();
        button.set_label(&shared::state_to_text(&timer.borrow().state));
    });

    button
}