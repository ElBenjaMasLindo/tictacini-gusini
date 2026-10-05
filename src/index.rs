use crate::gb;
use crate::interface::interface;

use adw::prelude::*; // GTK
use adw::{Application, ToolbarView, ApplicationWindow};

const SIZE: (i32, i32) = (500, 900); // x, y

pub fn index(app: &Application) {
    let tabs = interface::views();
    
    let program = ToolbarView::new();
    program.add_top_bar(&interface::bar(&tabs));
    program.set_content(Some(&tabs));
    
    gb::space(&program);

    let index = ApplicationWindow::builder()
        .default_width(SIZE.0) // x
        .default_height(SIZE.1) // y
        .application(app)
        .content(&program)
        .title(gb::NAME) 
        .build();

    index.present();
}
