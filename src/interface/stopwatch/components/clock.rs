use super::super::timer as T;
use super::shared;
use crate::gb;

use adw::gtk::{Label, pango, glib}; // GTK

use std::cell::RefCell;
use std::rc::Rc;

pub fn index(timer: Rc<RefCell<T::Timer>>) -> Label {
    let clock = Label::new(Some(&shared::duration_to_string(None)));
    style(&clock);
    
    gb::space(&clock);

    let timer = timer.clone();
    let clock_clone = clock.clone();

    let _source = glib::timeout_add_seconds_local(1, move || {
        clock_clone.set_label(
            &shared::duration_to_string(
                Some(timer.borrow().now())
            )
        ); 
        glib::ControlFlow::Continue
    });
    
    clock
}

fn style(label: &Label) {
    let attrs = pango::AttrList::new();
    let size = pango::AttrSize::new(50 * pango::SCALE);
    
    attrs.insert(size);
    label.set_attributes(Some(&attrs));
}
