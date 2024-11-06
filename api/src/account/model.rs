use std::fmt::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Error;
use crate::app::UtcDateTime;

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
            Ok(Login::Email(Email(v.to_string())))
        } else {
            Ok(Login::Nickname(Nickname(v.to_string())))
        }
    }

}

impl From<String> for Login {
    fn from(value: String) -> Self {
        if value.contains("@") {
            Self::Email(Email(value))
        } else {
            Self::Nickname(Nickname(value))
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

#[derive(Clone, Copy, Debug,Serialize, Deserialize, PartialEq, Default)]
pub struct UserId(pub i32);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Nickname(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Email(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: UserId,
    pub nickname: Nickname,
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