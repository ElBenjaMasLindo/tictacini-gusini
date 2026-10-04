use super::super::timer as T;
use crate::G;

use adw::prelude::*; // GTK
use adw::gtk::{Label, glib};

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(timer: Rc<RefCell<T::Timer>>) -> Label {
    let clock = Label::new(Some("00:00"));
    clock.set_size_request(250, 250);
    G::space(&clock);

    let timer = timer.clone();
    let clock_clone = clock.clone();

    let _source = glib::timeout_add_seconds_local(1, move || {
        clock_clone.set_label(&timer.borrow().now().as_secs().to_string()); 
        glib::ControlFlow::Continue
    });
    
    clock
}
