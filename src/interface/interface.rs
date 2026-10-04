use adw::{ViewSwitcher, ViewStack}; // GTK
use adw::gtk::Align;

use super::stopwatch::panel as stopwatch;
use super::stads::panel as stads;

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

    let names = ["Cronometro", "Estadisticas"];
    tabs.add_titled_with_icon(&stopwatch::panel(),
        Some(&names[0].to_lowercase()),
        names[0],
        "alarm-symbolic"
    );
    tabs.add_titled_with_icon(&stads::panel(),
        Some(&names[1].to_lowercase()),
        names[1],
        "open-menu-symbolic"
    );

    tabs
}