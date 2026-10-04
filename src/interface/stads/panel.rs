use adw::prelude::*; // GTK
use adw::gtk::{Box, Button, Label, Orientation};

use crate::global;

pub fn panel() -> Box {
    let pagina = Box::new(Orientation::Vertical, global::PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}