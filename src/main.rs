pub use adw::prelude::*; // GTK
pub use adw::{Application};

mod G;
mod index;
mod interface;

fn main() {
    let name_case = G::NAME.to_lowercase().replace(' ', "-").to_string();
    let app = Application::builder()
        .application_id(format!("com.elbenjamaslindo.{name_case}"))
        .build();
    
    app.connect_activate(index::index);

    app.run();
}