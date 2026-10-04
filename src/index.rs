use crate::global;
use crate::interface::interface;

use adw::prelude::*; // GTK
use adw::{Application, ToolbarView, ApplicationWindow};

const TAMAÑO: (i32, i32) = (800, 800); // x, y

pub fn index(app: &Application) {
    let tabs = interface::views();
    
    let program = ToolbarView::new();
    program.add_top_bar(&interface::bar(&tabs));
    program.set_content(Some(&tabs));
    
    program.set_margin_top(global::PADDING); // Padding
    program.set_margin_bottom(global::PADDING);
    program.set_margin_start(global::PADDING);
    program.set_margin_end(global::PADDING);

    let index = ApplicationWindow::builder()
        .default_width(TAMAÑO.0) // x
        .default_height(TAMAÑO.1) // y
        .application(app)
        .content(&program)
        .title(global::NAME)
        .build();

    index.present();
}
