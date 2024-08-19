use crate::resource::{Store};

pub struct City {
    pub score: Score,
    pub treas: Treas,
}

pub struct Treas {
    pub coins: u8,
    pub resources: Store,
}

#[derive(Debug, Default)]
pub struct Score {
    civilian: u8,
    science: u8,
    commercial: u8,
    guilds: u8,
    wonders: u8,
    tokens: u8,
    coins: u8,
    military: u8,
    total: u8,
}

