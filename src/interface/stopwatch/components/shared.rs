use crate::interface::timer as T;
use std::time::Duration;

pub fn state_to_text(state: &T::States) -> [String; 2] {
    match state {
        T::States::Other => return ["Descansar".to_string(), "Descanso".to_string()],
        T::States::Work => return ["Trabajar".to_string(), "Trabajo".to_string()],
    }
}

pub fn duration_to_string(time: Option<Duration>) -> String {
    let s = " : ";

    let Some(duration) = time else {
        return format!("0{s}0").to_string();
    };

    let secs = duration.as_secs();
    let mut time = vec![
        secs % 60,      // Segundos
        secs / 60 % 60, // Minutos
        secs / 60 / 60, // Horas
    ];

    time.pop_if(|x| *x <= 0);
    time.iter()
        .rev()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(s)
}
