use email_address::EmailAddress;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref LIST_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{1,20}$").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct MailingList {
    name: String,
    email: String,
}

impl MailingList {
    pub fn new(name: &str, email: &str) -> Option<Self> {
        if !EmailAddress::is_valid(email) {
            return None;
        }

        if !LIST_NAME_REGEX.is_match(name) {
            return None;
        }

        Some(MailingList {
            name: String::from(name),
            email: String::from(email),
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }
}