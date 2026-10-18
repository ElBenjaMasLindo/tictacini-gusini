use super::record;
use super::shared;
use crate::gb;
use crate::interface::timer as T;

use adw::gtk::{Box, Button, Orientation, ScrolledWindow};
use adw::prelude::*; // GTK

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(timer: Rc<RefCell<T::Timer>>, record: &Rc<RefCell<ScrolledWindow>>) -> Button {
    let button = Button::new();
    button.add_css_class("suggested-action");
    button.set_halign(gtk::Align::Center);
    button.set_size_request(150, 60);

    let timer = timer.clone();
    let record = record.clone();

    gb::space(&button);
    button.set_label(&shared::state_to_text(&timer.borrow().state)[0]);

    button.connect_clicked(move |button| {
        timer.borrow_mut().play();
        button.set_label(&shared::state_to_text(&timer.borrow().state)[0]);

        record.borrow_mut().set_child(None::<&adw::gtk::Widget>);

        let container = Box::new(Orientation::Vertical, gb::PADDING);
        record.borrow_mut().set_child(Some(&container));

        for i in timer.borrow().show().iter().rev() {
            container.append(&record::card(i.0, i.1));
        }
    });

    button
}
