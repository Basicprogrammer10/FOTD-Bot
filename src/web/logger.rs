use super::super::common::color::{self, Color};
use afire::{error::Result, middleware::Middleware, Request, Response, Server};

struct Logger;

impl Middleware for Logger {
    fn end(&self, req: &Result<Request>, _res: &Response) {
        if let Ok(req) = req {
            let text = format!(
                "[{}] {} {}",
                remove_address_port(&req.address),
                req.method,
                slash_path(&req.path),
            );

            color_print!(Color::Blue, "\x1b[2K\r{}", &text);
        }
    }
}

pub fn attach(server: &mut Server) {
    Logger.attach(server);
}

fn remove_address_port(address: &str) -> String {
    address.split(':').next().unwrap_or("null").to_string()
}

fn slash_path(path: &str) -> String {
    if path.starts_with('/') {
        return path.to_string();
    }
    format!("/{}", path)
}
