use std::collections::HashMap;

use super::{application::Application, Server};

/// NGINX server is a proxy to an application server.
pub struct NginxServer {
    application: Application,
    max_allowed_requests: u32,
    rate_limiter: HashMap<String, u32>,
}

impl NginxServer {
    pub fn new() -> Self {
        Self {
            application: Application,
            max_allowed_requests: 2,
            rate_limiter: HashMap::default(),
        }
    }

    pub fn check_rate_limiting(&mut self, url: &String) -> bool {
        let rate = self.rate_limiter.entry(url.clone()).or_insert(1);

        if *rate > self.max_allowed_requests {
            return false;
        }

        *rate += 1;
        return true;
    }
}

impl Server for NginxServer {
    fn handle_request(&mut self, url: &String, method: &str) -> (u16, String) {
        if !self.check_rate_limiting(url) {
            return (403, "Not Allowed".into());
        }

        return self.application.handle_request(url, method);
    }
}
