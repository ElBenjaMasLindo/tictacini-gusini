use adw::prelude::*; // GTK

#[allow(unused_imports)]
use adw::gtk::{Box, Button, Label, Orientation};

use crate::gb;

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, gb::PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}