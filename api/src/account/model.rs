use crate::prelude::*;
use std::fmt::Formatter;
use regex::Regex;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use validator::{ValidateEmail, ValidateLength, ValidateRegex, ValidationError};

pub const STARTING_RATING: Rating = 1500;

pub enum Login{
    Nickname(Nickname),
    Email(Email),
}

struct LoginVisitor;

impl<'de> Deserialize<'de> for Login {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LoginVisitor)
    }
}

impl <'de> de::Visitor<'de> for LoginVisitor {
    type Value = Login;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "valid str for login")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: Error
    {
        if v.contains("@") {
            Ok(Login::Email(v.to_string()))
        } else {
            Ok(Login::Nickname(v.to_string()))
        }
    }

}

impl Serialize for Login {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            Login::Nickname(v) => {
                serializer.serialize_str(v)
            }
            Login::Email(v) => {
                serializer.serialize_str(v)
            }
        }
    }
}

impl From<String> for Login {
    fn from(value: String) -> Self {
        if value.contains("@") {
            Self::Email(value)
        } else {
            Self::Nickname(value)
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct UserOptions {
    pub id: Option<UserId>,
    pub email: Option<Email>,
    pub nickname: Option<Nickname>,
}

impl UserOptions {
    pub fn with_id(&mut self, id: UserId) -> &mut Self {
        self.id = Some(id);

        self
    }

    pub fn with_email(&mut self, email: Email) -> &mut Self {
        self.email = Some(email);

        self
    }

    pub fn with_nickname(&mut self, nickname: Nickname) -> &mut Self {
        self.nickname = Some(nickname);

        self
    }
}
pub type UserId = u32;
pub type Nickname = String;
pub type Email = String;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: UserId,
    pub nickname: Nickname,
    pub rating: Rating,
    pub email: Email,
    pub password: String,
    pub settings: Settings,
    pub created_at: UtcDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub timezone: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timezone: "UTC".to_string()
        }
    }
}

pub type Rating = u16;

pub fn validate_login(login: &Login) -> Result<(), ValidationError> {
    match login {
        Login::Nickname(v) => {
            validate_nickname(v)
        },
        Login::Email(v) => {
            validate_email(v)
        },
    }
}

pub fn validate_nickname(nickname: &Nickname) -> Result<(), ValidationError> {
    if !nickname.validate_length(Some(3), Some(15), None) {
        return Err(ValidationError::new("invalid nickname length"));
    }

    if !nickname.validate_regex(Regex::new(r"^[a-zA-Z]+[a-zA-Z0-9]*$").unwrap()) {
        return Err(ValidationError::new("nickname should starts from letter and contains letters/numbers"));
    }

    Ok(())
}

pub fn validate_email(email: &Email) -> Result<(), ValidationError> {
    if !email.validate_email() {
        return Err(ValidationError::new("invalid email format"));
    }

    if !email.validate_length(None, Some(32), None) {
        return Err(ValidationError::new("invalid email length(32 max)"));
    }
    
    Ok(())
}

pub fn validate_password(password: &String) -> Result<(), ValidationError> {
    if !password.validate_length(Some(8), Some(128), None) {
        return Err(ValidationError::new("invalid password length"));
    }
    
    Ok(())
}