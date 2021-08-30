use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;

// TODO: Use these errors...
/// Errors that can occur when sending an email.
pub enum EmailError {
    MessageBuild,
    Transport,
    Authentication,
}

// Impl debug for EmailError
impl std::fmt::Debug for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmailError::MessageBuild => write!(f, "MessageBuild"),
            EmailError::Transport => write!(f, "Transport"),
            EmailError::Authentication => write!(f, "Authentication"),
        }
    }
}

pub struct Mailer {
    pub to: Vec<User>,
    pub from: User,
    pub subject: String,
    pub body: String,
    pub credentials: Creds,
    pub server: String,
}

pub struct User {
    pub email: String,
    pub name: String,
}

pub struct Creds {
    pub username: String,
    pub password: String,
}

/// Impl User
impl User {
    /// Make a new user
    pub fn new(email: String, name: String) -> User {
        User { email, name }
    }

    /// Get user as a string
    pub fn to_string(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }

    pub fn user_from_email(email: &str) -> User {
        let mut split = email.split('@');
        let name = split.next().unwrap();
        User::new(email.to_string(), name.to_string())
    }
}

// Impl Clone for user
impl Clone for User {
    fn clone(&self) -> User {
        User::new(self.email.clone(), self.name.clone())
    }
}

/// Impl Mailer
impl Mailer {
    /// Make a new mailer
    pub fn new(
        to: Vec<User>,
        from: User,
        subject: &str,
        body: &str,
        server: &str,
        username: &str,
        password: &str,
    ) -> Mailer {
        Mailer {
            to,
            from,
            subject: subject.to_string(),
            body: body.to_string(),
            credentials: Creds {
                username: username.to_string(),
                password: password.to_string(),
            },
            server: server.to_string(),
        }
    }

    /// Send to all users as individual emails
    pub fn send_all(&self) -> Result<u32, EmailError> {
        let mut count = 0;
        for user in &self.to {
            // Build the message
            let email = match Message::builder()
                .from((&self.from.to_string()).parse().unwrap())
                .to(user.to_string().parse().unwrap())
                .subject(&self.subject)
                .header(header::ContentType::TEXT_HTML)
                .body(String::from(&self.body).replace("{{NAME}}", &user.name))
            {
                // lil bodge {}
                Ok(email) => email,
                Err(_) => return Err(EmailError::MessageBuild),
            };

            // Get credentials for mail server
            let creds = Credentials::new(
                (&self.credentials.username).clone(),
                (&self.credentials.password).clone(),
            );

            // Open a remote connection to the mail server
            let mailer = match SmtpTransport::relay(&self.server) {
                Ok(mailer) => mailer.credentials(creds).build(),
                Err(_) => return Err(EmailError::Authentication),
            };

            // Send the email
            match mailer.send(&email) {
                Ok(_) => count += 1,
                Err(_) => return Err(EmailError::Transport),
            }
        }
        Ok(count)
    }
}
