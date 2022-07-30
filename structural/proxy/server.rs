mod application;
mod nginx;

pub use nginx::NginxServer;

pub trait Server {
    fn handle_request(&mut self, url: &String, method: &str) -> (u16, String);
}
