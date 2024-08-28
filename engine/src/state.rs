use std::cmp::Ordering;
use std::collections::HashMap;
use crate::{Deck, building, economy::{Discount, PriceList, Resource, Resources, Cost, PayScope}, effect, token, wonder, military::{Track}, Bonus, Nickname, Phase, COINS_PER_POINT, Victory, Coins, FIXED_RESOURCE_PRICE, ScientificSymbol, SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN, DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY};

pub struct State {
    pub phase: Phase,
    pub players: Players,
    pub deck: Deck,
    pub cities: HashMap<Nickname, City>,
    pub progress_tokens: Vec<token::Id>,
    pub buildings: Buildings,
    pub random_units: RandomUnits,
    pub interactive_units: Units,
    pub post_effects: Vec<Box<dyn effect::PostEffect>>,
    pub play_again: bool,
    pub result: Option<Result>,
}

impl State {
    pub fn me(&self) -> &City {
        self.cities.get(&self.players.me).unwrap()
    }

    pub fn me_mut(&mut self) -> &mut City {
        self.cities.get_mut(&self.players.me).unwrap()
    }

    pub fn enemy(&self) -> &City {
        self.cities.get(&self.players.enemy).unwrap()
    }

    pub fn enemy_mut(&mut self) -> &mut City {
        self.cities.get_mut(&self.players.enemy).unwrap()
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

        if self.enemy_mut().track.pos >= power {
            self.enemy_mut().track.pos -= power;

            return (fine, supremacy);
        }

        self.me_mut().track.pos += (power - self.enemy_mut().track.pos);
        self.enemy_mut().track.pos = 0;

        if self.me_mut().track.pos >= Track::CAPITAL_POS {
            self.me_mut().track.pos = Track::CAPITAL_POS;
            supremacy = true;
        }

        let zone_index = self.me_mut().track.get_zone_index();

        if zone_index > self.me_mut().track.max_zone {
            self.me_mut().track.max_zone = zone_index;
            fine = Track::ZONES[zone_index].2;
        }

        (fine, supremacy)
    }

    pub fn resolve_winner(&self) -> Nickname {
        let winner = match self.me().score.total.cmp(&self.enemy().score.total) {
            Ordering::Greater => &self.players.me,
            Ordering::Less => &self.players.enemy,
            Ordering::Equal => {
                match self.me().score.civilian.cmp(&self.enemy().score.commercial) {
                    Ordering::Greater => &self.players.me,
                    Ordering::Less => &self.players.enemy,
                    Ordering::Equal => {
                        if self.players.me == self.players.starts {
                            &self.players.me
                        } else {
                            &self.players.enemy
                        }
                    }
                }
            }
        };

        winner.clone()
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
    pub scientific_symbols: Vec<(ScientificSymbol, u8)>,
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

pub struct RandomUnits {
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

    pub fn has_fixed_resource_price(&self, r: &Resource) -> bool {
        self.resource_price[r] == FIXED_RESOURCE_PRICE
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

pub struct Players {
    pub starts: Nickname,
    pub me: Nickname,
    pub enemy: Nickname,
}

impl Players {
    pub fn next_turn(&mut self) {
        std::mem::swap(&mut self.me, &mut self.enemy);
    }

    pub fn set_turn(&mut self, turn: Nickname) {
        if self.me != turn {
            assert_eq!(self.enemy, turn);
            self.next_turn();
        }
    }
}

pub struct Buildings {
    pub discarded: Vec<building::Id>,
}

enum ScienceStatus {
    Regular,
    ProgressToken,
    Supremacy,
}