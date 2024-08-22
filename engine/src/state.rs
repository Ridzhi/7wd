use std::collections::HashMap;
use crate::{
    Nickname,
    Bonus,
    Phase,
    COINS_PER_POINT,
    resource::{Store},
    building,
    wonder,
    token,
    effect,
};

pub struct State<'a> {
    pub phase: Phase,
    pub turn: Nickname,
    pub cities: HashMap<Nickname, City>,
    pub interactive_effects: Vec<effect::InteractiveEffect<'a>>,
    pub interactive_units: Units<'a>,
}

impl State<'_> {
    pub fn me(&mut self) -> &mut City {
        self.cities.get_mut(&self.turn).unwrap()
    }

    pub fn enemy(&mut self) -> &mut City {
        let enemy = self.get_next_turn();

        self.cities.get_mut(&enemy).unwrap()
    }

    pub fn set_turn(&mut self, player: &Nickname) {
        if self.turn != *player {
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

#[derive(Default)]
pub struct Units<'a> {
    pub wonders: Option<&'a [wonder::Id]>,
    pub buildings: Option<&'a [building::Id]>,
    pub tokens: Option<&'a [token::Id]>,
}