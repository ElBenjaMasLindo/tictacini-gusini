use adw::prelude::*; // GTK
use adw::gtk::{Box, Widget, Orientation};

use crate::gb;

pub fn space_evenly(panel: impl IsA<Widget>, activate: (bool, bool)) -> Box {
    let index = Box::new(Orientation::Vertical, gb::PADDING);

    if activate.0 {
        let up = get_space();
        index.append(&up);
    }

    index.append(&panel);

    if activate.1 {
        let down = get_space();
        index.append(&down);
    };

    index
}

pub fn get_space() -> Box {
    let foo = Box::new(Orientation::Vertical, gb::PADDING);
    foo.set_vexpand(true);

    foo
}