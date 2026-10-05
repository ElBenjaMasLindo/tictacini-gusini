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
    let scroll = ScrolledWindow::builder()
        .build();

    scroll.set_halign(gtk::Align::Center);
    scroll.set_vexpand(true);
    scroll.set_policy(
        gtk::PolicyType::Never, // X
        gtk::PolicyType::Automatic // Y
    );

    scroll.set_hexpand(true);

    gb::space(&scroll);

    scroll
}

pub fn card(state: &T::States, time: Option<Duration>) -> Box {
    let card = Box::new(Orientation::Horizontal, gb::PADDING);
    card.set_width_request(340);
    card.add_css_class("card");
    
    let state = Label::new(Some(&shared::state_to_text(state)[1]));
    let label = Label::new(None);

    gb::space(&state);
    state.set_hexpand(true);
    state.set_halign(gtk::Align::Start);
    
    gb::space(&label);
    label.set_hexpand(true);
    
    match time {
        Some(foo) => label.set_label(&shared::duration_to_string(Some(foo))),
        None => label.set_label("..."),
    }

    card.append(&state);
    card.append(&label);

    card
}