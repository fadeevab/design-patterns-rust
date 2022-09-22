use crate::log::{error, info, warn};

pub fn run() {
    info("Application starts");
    warn("Something non-critical happened");
    error("Execution failed")
}
