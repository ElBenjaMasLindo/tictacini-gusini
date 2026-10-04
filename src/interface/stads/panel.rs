use adw::prelude::*; // GTK

#[allow(unused_imports)]
use adw::gtk::{Box, Button, Label, Orientation};

use crate::G;

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, G::PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}