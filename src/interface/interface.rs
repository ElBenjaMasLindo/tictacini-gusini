use crate::interface::timer as T;

use adw::gtk::Align;
use adw::{ViewStack, ViewSwitcher}; // GTK

use super::stads::panel as stads;
use super::stopwatch::panel as stopwatch;

use std::cell::RefCell;
use std::rc::Rc;

pub fn bar(tabs: &ViewStack) -> ViewSwitcher {
    ViewSwitcher::builder()
        .stack(&tabs)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .hexpand(false) // Lo Centramos
        .halign(Align::Center)
        .build()
}

pub fn views() -> ViewStack {
    let tabs = ViewStack::new();
    let timer = Rc::new(RefCell::new(T::Timer::new()));

    let names = ["Cronometro", "Estadisticas"];
    tabs.add_titled_with_icon(
        &stopwatch::panel(timer.clone()),
        Some(&names[0].to_lowercase()),
        names[0],
        "timer-symbolic",
    );
    tabs.add_titled_with_icon(
        &stads::panel(timer),
        Some(&names[1].to_lowercase()),
        names[1],
        "open-menu-symbolic",
    );

    tabs
}
