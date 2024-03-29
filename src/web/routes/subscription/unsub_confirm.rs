use std::fs;

use afire::prelude::*;
use rand::Rng;

use crate::{common::common, App, Arc};

pub fn attach(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/unsubscribe/confirm/real", move |req| {
        let code = match req.query.get("code") {
            Some(code) => common::decode_url_chars(&code),
            None => return Response::new().status(400).text("No Code supplied???"),
        };

        // Get email from hashmap
        let email = match app.unsub_codes.lock().get(&code) {
            Some(email) => email.clone().to_lowercase(),
            None => return Response::new().status(400).text("Invalid Code - Sorwy"),
        };

        if email.is_empty() {
            return Response::new().status(400).text("Invalid Email");
        }

        if code.is_empty() {
            return Response::new().status(400).text("Invalid Code");
        }

        // Remove from hashmap
        app.unsub_codes.lock().remove(&code);

        // Remove from database
        app.database
            .lock()
            .execute("DELETE FROM users WHERE email = ?", [&email])
            .unwrap();

        // Get a random Quote
        let quote = &QUOTES[rand::thread_rng().gen_range(0..QUOTES.len())];

        Response::new()
            .text(
                fs::read_to_string(
                    &app.config
                        .data_path
                        .join("web/unsubscribe/done/allDone.html"),
                )
                .unwrap_or_else(|_| {
                    "done. you ({{EMAIL}}) will no longer get amazing daily facts in your inbox :/"
                        .to_string()
                })
                .replace("{{EMAIL}}", &email)
                .replace("{{QUOTE}}", quote.quote)
                .replace("{{AUTHOR}}", quote.author)
                .replace("{{BASE_URL}}", &app.config.web_url),
            )
            .header("Content-Type", "text/html")
    });
}

/// ***Fun*** Quotes to show on unsubscribe page
const QUOTES: [Quote; 8] = [
    Quote {
        quote: "Go, throw yourself into the sea!",
        author: "Jesus",
    },
    Quote {
        quote: "Im not mad im just disappointed",
        author: "Every Parent Ever",
    },
    Quote {
        quote: "a threat to justice everywhere",
        author: "Martin Luther King JR",
    },
    Quote {
        quote: "worse than savage mobs",
        author: "Abraham Lincon",
    },
    Quote {
        quote: "Simba, I'm very disappointed in you.",
        author: "Mufasa",
    },
    Quote {
        quote: "You always have a choice. It's just that some people make the wrong one.",
        author: "Nicholas Sparks",
    },
    Quote {
        quote: "you egg! stab",
        author: "shakespeare",
    },
    Quote {
        quote: "skill issue,,,",
        author: "mr prez darren!?",
    },
];

// ✨ beyond ✨
struct Quote<'a> {
    quote: &'a str,
    author: &'a str,
}
