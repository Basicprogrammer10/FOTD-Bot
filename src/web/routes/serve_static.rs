use afire::*;
use std::fs;

use super::super::super::common::common;

/// Dir to find files to serve
const DATA_DIR: &str = "data/web";

pub fn add_route(server: &mut afire::Server) {
    server.all(|req| {
        let mut path = format!("{}{}", DATA_DIR, req.path.replace("/..", ""));

        // Add Index.html if path ends with /
        if path.ends_with('/') {
            path.push_str("index.html");
        }

        // Also add '/index.html' if path dose not end with a file
        if !path.split('/').last().unwrap_or_default().contains('.') {
            path.push_str("/index.html");
        }

        // Try to read File
        match fs::read(&path) {
            // If its found send it as response
            Ok(content) => Response::new_raw(
                200,
                content,
                vec![Header::new("Content-Type", common::get_type(&path))],
            ),

            // If not send 404.html
            Err(_) => Response::new_raw(
                404,
                fs::read(format!("{}/404.html", DATA_DIR))
                    .unwrap_or_else(|_| "Not Found :/".as_bytes().to_owned()),
                vec![Header::new("Content-Type", "text/html")],
            ),
        }
    });
}
