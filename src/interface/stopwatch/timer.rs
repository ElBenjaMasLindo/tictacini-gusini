use std::time::{Instant, Duration};

#[derive(Debug, PartialEq)]
pub enum States {
    Work,
    Other,
}

pub struct Timer {
    pub time: Vec<(States, Instant)>,
    pub state: States,
}

impl Timer {
    pub fn new() -> Timer {
        let foo: Vec<(States, Instant)> = vec![];
        
        Timer {
            time: foo,
            state: States::Work
        }
    }

    pub fn play(&mut self) {
        println!("{:?}", self.state);
        match self.state {
            States::Other => {
                self.time.push((States::Other, Instant::now()));
                self.state = States::Work;
            },
            States::Work => {
                self.time.push((States::Work, Instant::now()));
                self.state = States::Other;
            },
        }
        println!("{:?}", self.state);
    }

    pub fn show(&self) -> Vec<(&States, Option<Duration>)> {
        let mut corridas: Vec<(&States, Option<Duration>)> = vec![];
        
        for i in 0..self.time.len() {
            let actual = &self.time[i];

            let Some(siguiente) = &self.time.get(i + 1) else {
                corridas.push((&actual.0, None));
                continue;
            };
                
            corridas.push((&actual.0, Some(
                siguiente.1 // Tiempo Donde Acabo
                -
                actual.1 // Tiempo Donde Inicio
            )));
        };

        corridas
    }

    pub fn now(&self) -> Duration {
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