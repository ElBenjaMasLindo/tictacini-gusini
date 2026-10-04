use super::timer as T;
use super::components;

use adw::prelude::*; // GTK
use adw::gtk::{Box, Orientation};

use std::cell::RefCell;
use std::rc::Rc;

use crate::G;

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, G::PADDING);
    let timer = Rc::new(RefCell::new(T::Timer::new()));

    pagina.append(&components::clock::index(timer.clone()));
    pagina.append(&components::switcher::index(timer));
    
    pagina
}
