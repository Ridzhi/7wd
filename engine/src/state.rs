use std::collections::HashMap;
use crate::{
    Nickname,
    Bonus,
    COINS_PER_POINT,
    resource::{Store},
    building,
    wonder,
    token,
};

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
    pub coins: u8,
    pub resources: Store,
    pub score: Score,
    pub buildings: Vec<building::Id>,
    pub wonders: Vec<(wonder::Id, Option<building::Id>)>,
    pub tokens: Vec<token::Id>,
    pub chains: Vec<building::Id>,
}

impl City {
    pub fn bonus_rate(&self, b: Bonus) -> u8 {
        match b {
            Bonus::Resources => self.bonus_rate(Bonus::RawMaterials) + self.bonus_rate(Bonus::ManufacturedGoods),
            Bonus::RawMaterials => building::count_by_kind(&self.buildings, building::Kind::RawMaterials),
            Bonus::ManufacturedGoods => building::count_by_kind(&self.buildings, building::Kind::ManufacturedGoods),
            Bonus::Military => building::count_by_kind(&self.buildings, building::Kind::Military),
            Bonus::Commercial => building::count_by_kind(&self.buildings, building::Kind::Commercial),
            Bonus::Civilian => building::count_by_kind(&self.buildings, building::Kind::Civilian),
            Bonus::Science => building::count_by_kind(&self.buildings, building::Kind::Scientific),
            Bonus::Wonder => {
                self.wonders
                    .iter()
                    .filter(|(_, building)| building.is_some())
                    .count() as u8
            }
            Bonus::Coin => self.coins / COINS_PER_POINT,
        }
    }
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

