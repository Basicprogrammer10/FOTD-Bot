use afire::*;

pub use super::email::{quick_email, Auth};
mod routes;

pub fn start(
    ip: &str,
    port: u16,
    email_auth: Auth,
    base_url: String,
    template_path: String,
    user_path: String,
) {
    let mut server: Server = Server::new(ip, port);

    // Add Logger and Rate Limiter
    Logger::attach(&mut server, Logger::new(Level::Info, None, true));
    // RateLimiter::attach(&mut server, 10, 30);

    // Serve Static files from DATA_DIR
    routes::serve_static::add_route(&mut server);

    // Process Unsub requests
    routes::unsub::add_route(
        &mut server,
        email_auth.clone(),
        base_url.clone(),
        template_path.clone(),
        user_path.clone(),
    );

    // Process Sub requests
    routes::sub::add_route(
        &mut server,
        email_auth.clone(),
        base_url.clone(),
        template_path.clone(),
        user_path.clone(),
    );

    server.start();
}
