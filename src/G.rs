use gtk::prelude::*;
use gtk::Widget;

pub const NAME: &str = "Tictacini Gusini";
pub const PADDING: i32 = 15;

pub fn space(widget: &impl IsA<Widget>) {
    widget.set_margin_top(PADDING);
    widget.set_margin_bottom(PADDING);
    widget.set_margin_start(PADDING);
    widget.set_margin_end(PADDING);
}