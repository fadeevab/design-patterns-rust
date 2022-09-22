use once_cell::sync::OnceCell;
use std::fmt::Display;

// OnceCell allows to set a logger later but only once.
static LOGGER: OnceCell<Box<dyn Log + Send + Sync>> = OnceCell::new();

#[derive(Debug)]
pub struct SetLoggerError;

pub fn set_boxed_logger(logger: Box<dyn Log + Sync + Send>) -> Result<(), SetLoggerError> {
    if LOGGER.set(logger).is_err() {
        return Err(SetLoggerError);
    }

    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd)]
pub enum Level {
    Error,
    Warn,
    Info,
}

pub trait Log {
    fn enabled(&self, level: &Level) -> bool;
    fn log(&self, level: &Level, message: &str);
}

fn log(level: Level, message: &str) {
    if let Some(logger) = LOGGER.get() {
        if logger.enabled(&level) {
            logger.log(&level, message);
        }
    }
}

pub fn error(message: &str) {
    log(Level::Error, message);
}

pub fn warn(message: &str) {
    log(Level::Warn, message);
}

pub fn info(message: &str) {
    log(Level::Info, message);
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Level::Error => "E",
                Level::Warn => "W",
                Level::Info => "I",
            }
        )
    }
}
