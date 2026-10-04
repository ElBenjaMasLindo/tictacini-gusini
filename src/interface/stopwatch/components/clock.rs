use super::super::timer as T;
use crate::gb;

use adw::prelude::*; // GTK
use adw::gtk::{Label, CssProvider, glib};

use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

fn duration_to_string(input: Option<Duration>) -> String {
    let Some(duration) = input else {
        return "0  .  0  .  0".to_string();
    };

    let secs = duration.as_secs();
    let time = (
        secs % 60, // Segundos
        secs / 60 % 60, // Minutos
        secs / 60 / 60, // Horas
    );
    
    format!("{}  .  {}  .  {}", time.2, time.1, time.0)
}

pub fn index(timer: Rc<RefCell<T::Timer>>) -> Label {
    let clock = Label::new(Some(&duration_to_string(None)));
    clock.add_css_class("title-1");
    
    gb::space(&clock);

    let timer = timer.clone();
    let clock_clone = clock.clone();

    let _source = glib::timeout_add_seconds_local(1, move || {
        clock_clone.set_label(&duration_to_string(Some(timer.borrow().now()))); 
        glib::ControlFlow::Continue
    });
    
    clock
}
