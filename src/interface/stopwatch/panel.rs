use super::components;
use crate::interface::timer as T;

use adw::gtk::{Box, Orientation};
use adw::prelude::*; // GTK

use std::cell::RefCell;
use std::rc::Rc;

use crate::gb;

pub fn panel(timer: Rc<RefCell<T::Timer>>) -> Box {
    let pagina = Box::new(Orientation::Vertical, gb::PADDING);
    let record = Rc::new(RefCell::new(components::record::index(timer.clone())));

    pagina.append(&components::clock::index(timer.clone()));
    pagina.append(&record.borrow().clone());
    pagina.append(&components::switcher::index(timer.clone(), &record));

    pagina
}
