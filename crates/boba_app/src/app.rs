use std::time::Instant;

pub struct BobaApp {
    instant: Instant,
}

impl BobaApp {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        println!("Update App {:?}", 1. / self.instant.elapsed().as_secs_f64());
        self.instant = Instant::now();
    }
}

pub trait BobaRunner {
    fn run(self, app: BobaApp);
}
