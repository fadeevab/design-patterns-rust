mod app;
mod log;
mod simple_logger;

use log::{info, Level};

fn main() {
    info("This log is not going to be displayed");

    simple_logger::init(Level::Info);

    app::run();
}
