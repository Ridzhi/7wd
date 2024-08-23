use std::collections::HashMap;
use crate::{building, economy, effect, token, wonder, Bonus, Nickname, Phase, COINS_PER_POINT};
use crate::economy::{PriceList, Resource, Resources};

pub struct State {
    pub phase: Phase,
    pub turn: Nickname,
    pub cities: HashMap<Nickname, City>,
    pub post_effects: Vec<Box<dyn effect::PostEffect>>,
    pub interactive_units: Units,
}

impl State {
    pub fn me(&mut self) -> &mut City {
        self.cities.get_mut(&self.turn).unwrap()
    }

    pub fn enemy(&mut self) -> &mut City {
        let enemy = self.get_next_turn();

        self.cities.get_mut(&enemy).unwrap()
    }

    pub fn set_turn(&mut self, player: Nickname) {
        if self.turn != player {
            self.next_turn()
        }
    }

    pub fn next_turn(&mut self) {
        let enemy = self.get_next_turn();
        self.turn = enemy;
    }

    fn get_next_turn(&self) -> Nickname {
        self.cities
            .keys()
            .find(|&k| !k.eq(&self.turn))
            .unwrap()
            .clone()
    }
}

#[derive(Default)]
pub struct City {
    pub coins: u8,
    pub resources: Resources,
    pub score: Score,
    pub buildings: Vec<building::Id>,
    pub wonders: Vec<(wonder::Id, Option<building::Id>)>,
    pub tokens: Vec<token::Id>,
    pub chains: Vec<building::Id>,
    pub bank: Bank,
    pub discounter: economy::Discounter,
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

#[derive(Default)]
pub struct Units {
    pub wonders: Vec<wonder::Id>,
    pub buildings: Vec<building::Id>,
    pub tokens: Vec<token::Id>,
}

#[derive(Debug, Default)]
pub struct Bank {
    pub discard_reward: u8,
    pub building_price: PriceList<building::Id>,
    pub wonder_price: PriceList<wonder::Id>,
    pub resource_price: PriceList<Resource>,
}
