use crate::log::{self, Level, Log};

struct SimpleLogger {
    max_level: Level,
}

impl Log for SimpleLogger {
    fn enabled(&self, level: &Level) -> bool {
        *level <= self.max_level
    }

    fn log(&self, level: &Level, message: &str) {
        println!("[{}] {}", level, message);
    }
}

pub fn init(max_level: Level) {
    log::set_boxed_logger(Box::new(SimpleLogger { max_level }))
        .expect("Logger has been already set");
}
