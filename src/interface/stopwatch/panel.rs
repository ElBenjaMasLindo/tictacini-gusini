use super::timer as T;
use super::components;

use adw::prelude::*; // GTK
use adw::gtk::{Box, Button, Label, Orientation};

use crate::G;

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, G::PADDING);
    let mut timer = T::Timer::new();

    pagina.append(&components::clock(&mut timer));
    pagina.append(&components::switch(&mut timer));
    
    pagina
}
