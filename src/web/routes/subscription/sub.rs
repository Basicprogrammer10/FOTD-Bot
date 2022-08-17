use afire::{prelude::*, Query};
use rand::Rng;
use std::fs;
use std::sync::Arc;

use crate::{common::common, web::quick_email, App};

/// Dir to find files to serve
const DATA_DIR: &str = "data/web";

pub fn attach(server: &mut afire::Server, app: Arc<App>) {
    server.route(Method::POST, "/subscribe", move |req| {
        let query = Query::from_body(req.body_string().unwrap()).unwrap();

        // Get email address
        let email = match query.get("email") {
            Some(email) => {
                if email.is_empty() {
                    return Response::new().status(400).text("Email is empty");
                }
                common::decode_url_chars(&email).to_lowercase()
            }
            None => return Response::new().status(400).text("Invalid Email"),
        };

        // If email is already subscribed dont send email etc.
        let content = fs::read_to_string(&app.config.user_path).unwrap_or_default();
        if content.contains(&email) {
            return Response::new().text("Your Already Subscribed!\nNo need to subscribe again!");
        }

        // Get confirm Url
        let random_chars = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .collect::<Vec<u8>>();

        // Convert to string
        let random_chars = String::from_utf8(random_chars).unwrap();

        // Add to hashmap
        app.sub_codes
            .write()
            .unwrap()
            .insert(random_chars.clone(), email.clone());
        let confirm_url = &format!(
            "{}/subscribe/confirm?code={}",
            app.config.web_url, random_chars
        );

        // Try to read File
        let to_send =
            match fs::read_to_string(format!("{}/subscribe.html", app.config.template_path)) {
                Ok(content) => content,
                Err(_) => "Subscribe: {{URL}}".to_string(),
            };

        quick_email(
            &app.config.web_auth,
            email.clone(),
            "FOTD BOT Subscription".to_string(),
            to_send.replace("{{URL}}", confirm_url),
        );

        Response::new()
            .text(
                fs::read_to_string(format!("{}/subscribe/done/index.html", DATA_DIR))
                    .unwrap_or_else(|_| "done. email sent to {{EMAIL}} to confirm sub!".to_string())
                    .replace("{{EMAIL}}", &email),
            )
            .header("Content-Type", "text/html")
    });
}