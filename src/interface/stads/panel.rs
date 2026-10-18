use crate::interface::timer as T;

use std::cell::RefCell;
use std::rc::Rc;

use adw::prelude::*; // GTK

#[allow(unused_imports)]
use adw::gtk::{Box, Button, Label, Orientation};

use crate::gb;

pub fn panel(timer: Rc<RefCell<T::Timer>>) -> Box {
    let pagina = Box::new(Orientation::Vertical, gb::PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}
