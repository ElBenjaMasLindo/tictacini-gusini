use super::super::timer as T;
use std::time::Duration;

pub fn state_to_text(state: &T::States) -> [String; 2] {
    match state {
        T::States::Other => return ["Descansar".to_string(), "Descanso".to_string()],
        T::States::Work => return ["Trabajar".to_string(), "Trabajo".to_string()]
    }
}

pub fn duration_to_string(time: Option<Duration>) -> String {
    let s = "  :  ";
    
    let Some(duration) = time else {
        return format!("0{s}0{s}0").to_string();
    };

    let secs = duration.as_secs();
    let time = (
        secs % 60, // Segundos
        secs / 60 % 60, // Minutos
        secs / 60 / 60, // Horas
    );
    
    format!("{}{s}{}{s}{}", time.2, time.1, time.0)
}