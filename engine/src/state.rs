use std::collections::HashMap;
use crate::{
    Nickname,
    Bonus,
    resource::{Store},
    building,
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
    pub score: Score,
    pub treas: Treas,
    pub chains: Vec<building::Id>
}

impl City {
    // pub fn bonus_rate(&self, b: Bonus) -> u8 {
    //     match b {
    //         Bonus::Resources => self.bonus_rate(Bonus::RawMaterials) + self.bonus_rate(Bonus::ManufacturedGoods),
    //         Bonus::RawMaterials => by_group(&self.buildings, BGroup::RawMaterials).len() as u8,
    //         Bonus::ManufacturedGoods => by_group(&self.buildings, BGroup::ManufacturedGoods).len() as u8,
    //         Bonus::Military => by_group(&self.buildings, BGroup::Military).len() as u8,
    //         Bonus::Commercial => by_group(&self.buildings, BGroup::Commercial).len() as u8,
    //         Bonus::Civilian => by_group(&self.buildings, BGroup::Civilian).len() as u8,
    //         Bonus::Science => by_group(&self.buildings, BGroup::Scientific).len() as u8,
    //         Bonus::Wonder => {
    //             self.wonders
    //                 .iter()
    //                 .filter(|(_, bid)| bid.is_some())
    //                 .count() as u8
    //         }
    //         Bonus::Coin => self.coins / COINS_PER_POINT,
    //     }
    // }
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

