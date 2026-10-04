use adw::prelude::*; // GTK
use adw::{Application, ToolbarView, ApplicationWindow, ViewStack, ViewSwitcher, glib};
use adw::gtk::{Box, Button, Label, Orientation, Align};

use std::time::{Instant, Duration};

const NAME: &str = "Tictacini Gusini";
const PADDING: i32 = 15;
const TAMAÑO: (i32, i32) = (800, 800); // x, y

fn main() {
    let name_case = NAME.to_lowercase().replace(' ', "-").to_string();
    let app = Application::builder()
        .application_id(format!("com.elbenjamaslindo.{name_case}"))
        .build();
    
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
    
    program.set_margin_top(PADDING); // Padding
    program.set_margin_bottom(PADDING);
    program.set_margin_start(PADDING);
    program.set_margin_end(PADDING);

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
        
        .hexpand(false) // Lo Centramos
        .halign(Align::Center)
        
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

enum States {
    Init,
    Work,
    Other,
}

struct Timer {
    time: Vec<(States, Instant)>,
    state: States,
}

impl Timer {
    fn new() -> Timer {
        let foo: Vec<(States, Instant)> = vec![];
        
        Timer {
            time: foo,
            state: States::Init
        }
    }

    fn play(&mut self) {
        match self.state {
            States::Init => {
                self.time.push((States::Init, Instant::now()));
                self.state = States::Work;
            },
            States::Other => {
                self.time.push((States::Other, Instant::now()));
                self.state = States::Work;
            },
            States::Work => {
                self.time.push((States::Work, Instant::now()));
                self.state = States::Other;
            },
        }
    }

    fn show(&self) -> (Duration, Vec<(&States, Option<Duration>)>) {
        let mut corridas: Vec<(&States, Option<Duration>)> = vec![];
        
        for i in 0..self.time.len() {
            let actual = &self.time[i];
            let Some(siguiente) = &self.time.get(i + 1) else {
                corridas.push((&self.state, None));
                continue;
            };
                
            corridas.push((&self.state, Some(
                siguiente.1 // Tiempo Donde Acabo
                -
                actual.1 // Tiempo Donde Inicio
            )));
        };

        (Self::now(&self), corridas)
    }

    fn now(&self) -> Duration {
        let total: Duration;
        match self.time.first() {
            None => total = Duration::default(),
            Some(data) => {
                total = data.1.elapsed();
            }
        }
        
        total
    }
}

fn state_to_text(state: States) -> String {
    match state {
        States::Init => return "Arrancar".to_string(),
        States::Other => return "Descansando".to_string(),
        States::Work => return "Trabajando".to_string()
    }
}

fn cronometro() -> Box {
    let pagina = Box::new(Orientation::Vertical, PADDING);

    let mut timer = Timer::new();

    let estado = Label::new(Some("Laburando"));
    let clock = Label::new(Some("00:00"));
    let _source = glib::timeout_add_seconds_local(1, || {
        let time = format!("{}:{}", timer.now().as_secs() / 60, timer.now().as_secs() % 60);
        clock.set_label(&time); 
        glib::ControlFlow::Continue
    });
    
    let switch = Button::new();
    switch.set_label("Arrancar");
    switch.connect_clicked(|switch| {
        timer.play();
        
        let label = &state_to_text(timer.state);
        switch.set_label(label);
        estado.set_label(label);
    });

    pagina.append(&estado);
    pagina.append(&clock);
    pagina.append(&switch);
    
    pagina
}

fn estadisticas() -> Box {
    let pagina = Box::new(Orientation::Vertical, PADDING);
    pagina.append(&Label::new(Some("Segunda")));
    pagina
}