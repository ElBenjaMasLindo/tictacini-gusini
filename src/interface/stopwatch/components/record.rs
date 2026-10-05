use super::super::timer as T;
use super::shared;
use crate::gb;

use adw::prelude::*; // GTK
use adw::gtk::{ScrolledWindow, Box, Orientation};
use gtk::Label;

use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(_timer: Rc<RefCell<T::Timer>>) -> ScrolledWindow {
    let scrool = ScrolledWindow::new();
    scrool.set_halign(gtk::Align::Center);

    gb::space(&scrool);

    scrool
}

fn card(state: &T::States, time: Option<Duration>) -> Box {
    let card = Box::new(Orientation::Horizontal, gb::PADDING);
    card.add_css_class("card");
    gb::space(&card);
    
    let state = Label::new(Some(&shared::state_to_text(state)));
    let label = Label::new(None);
    
    match time {
        Some(foo) => label.set_label(&shared::duration_to_string(Some(foo))),
        None => label.set_label("..."),
    }

    card.append(&state);
    card.append(&label);

    card
}