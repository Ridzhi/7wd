use std::cmp::Ordering;
use std::collections::HashMap;
use crate::{building, economy::{Discount, PriceList, Resource, Resources, Cost, PayScope}, effect, token, wonder, military::{Track}, Bonus, Nickname, Phase, COINS_PER_POINT, Victory, Coins};

pub struct State {
    pub phase: Phase,
    pub first_turn: Nickname,
    pub turn: Nickname,
    pub cities: HashMap<Nickname, City>,
    pub post_effects: Vec<Box<dyn effect::PostEffect>>,
    pub interactive_units: Units,
    pub result: Option<Result>,
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

    pub fn over(&mut self, victory: Victory, winner: Nickname) {
        if self.phase == Phase::Over {
            return;
        }


        self.phase = Phase::Over;
        self.result = Some(Result {
            winner,
            victory,
        });
        // if winner.is_none() {
        //     if self.me().score.total != self.enemy().score.total {
        //         winner = Some(
        //             if self.me().score.total > self.enemy().score.civilian {
        //                 self.me().name
        //             } else {
        //                 self.enemy().name
        //             }
        //         );
        //     }
        // }

        // refresh cities
    }

    pub fn move_conflict_pawn(&mut self, power: u8) -> (Coins, bool) {
        let mut fine: Coins = 0;
        let mut supremacy = false;

        if self.enemy().track.pos >= power {
            self.enemy().track.pos -= power;

            return (fine, supremacy);
        }

        self.me().track.pos += (power - self.enemy().track.pos);
        self.enemy().track.pos = 0;

        if self.me().track.pos >= Track::CAPITAL_POS {
            self.me().track.pos = Track::CAPITAL_POS;
            supremacy = true;
        }

        let zone_index = self.me().track.get_zone_index();

        if zone_index > self.me().track.max_zone {
            self.me().track.max_zone = zone_index;
            fine = Track::ZONES[zone_index].2;
        }

        (fine, supremacy)
    }

    pub fn resolve_winner(&mut self) -> Nickname {
        let winner = match self.me().score.total.cmp(&self.enemy().score.total) {
            Ordering::Greater => &self.me().name,
            Ordering::Less => &self.enemy().name,
            Ordering::Equal => {
                match self.me().score.civilian.cmp(&self.enemy().score.commercial) {
                    Ordering::Greater => &self.me().name,
                    Ordering::Less => &self.enemy().name,
                    Ordering::Equal => {
                        if self.me().name == self.first_turn {
                            &self.me().name
                        } else {
                            &self.enemy().name
                        }
                    }
                }
            }
        };

        winner.clone()
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
    pub name: Nickname,
    pub coins: u8,
    pub resources: Resources,
    pub score: Score,
    pub buildings: Vec<building::Id>,
    pub wonders: Vec<(wonder::Id, Option<building::Id>)>,
    pub tokens: Vec<token::Id>,
    pub chains: Vec<building::Id>,
    pub bank: Bank,
    pub track: Track,
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
    pub discounts: Vec<Discount>,
}

impl Bank {
    pub fn get_price(&self, scope: PayScope, cost: Cost) -> u8 {
        0
    }

    fn get_resources_ordered_by_price(&self) -> Vec<Resource> {
        let mut items = self.resource_price.iter().collect::<Vec<_>>();
        items.sort_by(|&a, &b| b.1.cmp(a.1));

        items.iter().map(|item| *item.0).collect()
    }

    fn discount(&self, scope: PayScope, cost: &mut Cost) {
        let priority = self.get_resources_ordered_by_price();

        self.discounts.iter()
            .filter(|&item| item.scope == PayScope::Global || item.scope == scope)
            .for_each(|discount| {
                discount.apply(cost, &priority);
            });
    }
}

struct Result {
    pub winner: Nickname,
    pub victory: Victory,
}