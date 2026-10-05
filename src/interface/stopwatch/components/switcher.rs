use super::super::timer as T;
use super::record;
use super::shared;
use crate::gb;

use adw::prelude::*; // GTK
use adw::gtk::{Button, Box, ScrolledWindow, Orientation};

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(timer: Rc<RefCell<T::Timer>>, record: &Rc<RefCell<ScrolledWindow>>) -> Button {
    let button = Button::new();
    button.add_css_class("suggested-action");
    button.set_halign(gtk::Align::Center);
    
    let timer = timer.clone();
    let record = record.clone();

    gb::space(&button);
    button.set_label(&shared::state_to_text(&timer.borrow().state));

    button.connect_clicked(move |button| {
        timer.borrow_mut().play();
        button.set_label(&shared::state_to_text(&timer.borrow().state));

        record.borrow_mut().set_child(None::<&adw::gtk::Widget>);
        
        let container = Box::new(Orientation::Vertical, gb::PADDING);
        record.borrow_mut().set_child(Some(&container));

        for i in timer.borrow().show().iter().rev() {
            container.append(&record::card(i.0, i.1));
            println!("{i:?}")
        }
    });

    button
}