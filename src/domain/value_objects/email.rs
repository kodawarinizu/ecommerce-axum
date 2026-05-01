use std::sync::OnceLock;
use regex::Regex;

use crate::domain::errors::DomainError;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

#[derive(Debug, Clone)]
pub struct Email(String);
impl Email {
    pub fn new (email: String) -> Result<Self, DomainError> {
        let re = email_regex();
        if re.captures(&email).is_some() {
            return Ok(Self(email.to_lowercase()));
        }
        Err(DomainError::InvalidEmail(format!("{} no es un email válido", email)))
    }
    pub fn value (&self) -> &str {
        self.0.as_str()
    }
}