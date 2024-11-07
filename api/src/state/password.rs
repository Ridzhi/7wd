use crate::prelude::*;

use pwhash::bcrypt::{hash, verify};


pub struct Password;

impl Password {
    pub fn hash(&self, raw_password: String) -> Result<String> {
        Ok(hash(raw_password)?)
    }

    pub fn verify(&self, raw_password: &str, hash_password: &str) -> bool {
        verify(raw_password, hash_password)
    }
}