use super::super::timer as T;
use std::time::Duration;

pub fn state_to_text(state: &T::States) -> String {
    match state {
        T::States::Init => return "Arrancar".to_string(),
        T::States::Other => return "Descansando".to_string(),
        T::States::Work => return "Trabajando".to_string()
    }
}

pub fn duration_to_string(time: Option<Duration>) -> String {
    let Some(duration) = time else {
        return "0  .  0  .  0".to_string();
    };

    let secs = duration.as_secs();
    let time = (
        secs % 60, // Segundos
        secs / 60 % 60, // Minutos
        secs / 60 / 60, // Horas
    );
    
    format!("{}  .  {}  .  {}", time.2, time.1, time.0)
}