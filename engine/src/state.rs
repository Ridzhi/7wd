use std::collections::HashMap;
use crate::{Nickname};
use crate::resource::{Store};

pub struct State {
    pub turn: Nickname,
    pub cities: HashMap<Nickname, City>,
}

impl State {
    pub fn me(&mut self) -> &mut City {
        self.cities.get_mut(&self.turn).unwrap()
    }

    pub fn enemy(&mut self) -> &mut City {
        let enemy_key = self.cities
            .keys()
            .find(|&k| !k.eq(&self.turn))
            .unwrap()
            .clone();

        self.cities.get_mut(&enemy_key).unwrap()
    }
}

#[derive(Default)]
pub struct City {
    pub score: Score,
    pub treas: Treas,
}

#[derive(Default)]
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

