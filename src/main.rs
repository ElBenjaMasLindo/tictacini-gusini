use adw::ffi::AdwToolbarView;
#[allow(unused_imports)]
use adw::prelude::*;
#[allow(unused_imports)]
use adw::{Application, ToolbarView, ApplicationWindow, HeaderBar, ViewStack, ViewSwitcher, glib};
#[allow(unused_imports)]
use adw::gtk::{Box, Button, Label, Orientation};

const NAME: &str = "Tictacini Gusini";
const PADDING: i32 = 15;
const TAMAÑO: (i32, i32) = (800, 800); // x, y

fn main() {
    let name_case = NAME.to_lowercase().replace(' ', "-").to_string();
    let app = Application::builder()
        .application_id(format!("com.elbenjamaslindo.{name_case}")).build();
    
    app.connect_activate(index);

    app.run();
}

fn index(app: &Application) {
    let tabs = views();
    let toolbar = bar(&tabs);
    let content = &tabs;
    
    let program = ToolbarView::new();
    program.add_top_bar(&toolbar);
    program.set_content(Some(content));

    let index = ApplicationWindow::builder()
        .default_width(TAMAÑO.0) // x
        .default_height(TAMAÑO.1) // y
        .application(app)
        .content(&program)
        .title(NAME)
        .build();

    index.present();
}

fn bar(tabs: &ViewStack) -> ViewSwitcher {
    ViewSwitcher::builder()
        .stack(&tabs)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build()
}

fn views() -> ViewStack {
    let tabs = ViewStack::new();

    let names = ["Cronometro", "Estadisticas"];
    tabs.add_titled_with_icon(&cronometro(),
        Some(&names[0].to_lowercase()),
        names[0],
        "alarm-symbolic"
    );
    tabs.add_titled_with_icon(&estadisticas(),
        Some(&names[1].to_lowercase()),
        names[1],
        "open-menu-symbolic"
    );

    tabs
}

fn cronometro() -> Box {
    let pagina = Box::new(Orientation::Vertical, PADDING);
    pagina.append(&Label::new(Some("Primera")));
    pagina
}

fn estadisticas() -> Box {
    let pagina = Box::new(Orientation::Vertical, PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}