use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::{Deck, building, economy::{Discount, PriceList, Resource, Resources, Cost, PayScope}, effect, token, wonder, military::{Track}, Bonus, Nickname, Phase, COINS_PER_POINT, Victory, Coins, FIXED_RESOURCE_PRICE, ScientificSymbol, SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN, DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY, Action, Error, Age, DEFAULT_DISCARD_REWARD, DEFAULT_RESOURCE_PRICE, STARTING_CITY_COINS, get_layout, BaseUnit};
use crate::building::Kind;
use crate::deck::Layout;
use crate::effect::PostEffect;
use crate::player::Finisher;

#[derive(Default, Debug)]
pub struct State {
    pub age: Age,
    pub phase: Phase,
    pub players: Players,
    pub cities: HashMap<Nickname, City>,
    pub progress_tokens: [Option<token::Id>; 5],
    pub buildings: Buildings,
    pub interactive_units: Units,
    pub post_effects: Vec<PostEffect>,
    pub play_again: bool,
    pub finish: Option<Finish>,

    // clients invisible
    pub deck: Deck,
    pub random_units: RandomUnits,
}

impl State {
    pub fn from(actions: Vec<Action>) -> Result<Self, Error> {
        let mut s = Self::default();

        for action in actions {
            action.apply(&mut s)?;
        }

        Ok(s)
    }
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

    pub fn over(&mut self, finisher: Finisher, victory: Victory) {
        if self.phase == Phase::Over {
            return;
        }

        self.phase = Phase::Over;

        let winner= match finisher {
            Finisher::Winner(w) => w,
            Finisher::Loser(l) => {
                if l == self.players.me {
                    self.players.enemy
                } else {
                    self.players.me
                }
            }
        };

        self.finish = Some(Finish {
            winner,
            victory,
        });
    }

    pub fn pay(&mut self, scope: PayScope, cost: Cost) -> Result<(), Error> {
        let cost_coins = cost.coins;
        let price = self.me().bank.get_price(scope, cost);

        if price > self.me().coins {
            return Err(Error::NotEnoughCoins);
        }

        self.me_mut().coins -= price;

        if self.enemy().tokens.contains(&token::Id::Economy) {
            self.enemy_mut().coins += (price - cost_coins);
        }

        Ok(())
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
            Ordering::Greater => self.players.me,
            Ordering::Less => self.players.enemy,
            Ordering::Equal => {
                match self.me().score.civilian.cmp(&self.enemy().score.commercial) {
                    Ordering::Greater => self.players.me,
                    Ordering::Less => self.players.enemy,
                    Ordering::Equal => {
                        if self.players.me == self.players.starts {
                            self.players.me
                        } else {
                            self.players.enemy
                        }
                    }
                }
            }
        };

        winner
    }

    pub fn after(&mut self) {
        if self.phase == Phase::Over {
            return;
        }

        let has_post_effects = self.post_effects.len() > 0;
        let is_over = self.age.is_last()
            && self.phase == Phase::Turn
            && self.deck.is_empty()
            && !has_post_effects;

        if is_over {
            self.over(Finisher::Winner(self.resolve_winner()), Victory::Civilian);
            return;
        }

        // post effects has own logic to set turn
        // we resolve turn and use this to fallback turn after post effects if needed
        self.resolve_next_turn();

        if !has_post_effects {
            if self.deck.is_empty() && !self.age.is_last() {
                self.age.next();
                self.deck = Deck::new(get_layout(self.age), self.random_units.buildings[&self.age].clone())
            }
        }

        self.refresh_buildings();
        self.refresh_cities();
        // refresh_building
        // refresh cities

    }

    fn refresh_buildings(&mut self) {
        self.buildings.layout = self.deck.get_public_layout();
        self.buildings.playable = self.deck.get_playable_buildings();
    }

    fn refresh_cities(&mut self) {
        let turn = self.players.me;

        vec![self.players.me, self.players.enemy].into_iter()
            .for_each(|p| {
                self.players.set_turn(p);
                // @TODO remove clone
                let playable = self.buildings.playable.clone();
                self.me_mut().refresh_buildings_price(playable);
                self.me_mut().refresh_wonders_price();
                let score = get_score(p, self);
                self.me_mut().score = score;
                // сделать получение скора и присваивание снаружи
                // self.me_mut().refresh_score();
                // self.me_mut()
            })
    }

    fn resolve_next_turn(&mut self) {
        if self.deck.is_empty() && !self.age.is_last() {
            self.phase = Phase::WhoBeginsTheNextAgeSelection;
            self.play_again = false;

            // if military parity last player continue
            if self.me().track.pos == self.enemy().track.pos {
                return;
            }

            // if enemy no have military advantage turn moves to him, otherwise me stay
            if self.enemy().track.pos == 0 {
                self.players.next_turn();
                return;
            }
        }

        if self.play_again {
            self.play_again = false;
            return;
        }

        self.players.next_turn();
    }
}

#[derive(Debug)]
pub struct City {
    pub coins: Coins,
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

    pub fn refresh_buildings_price(&mut self, target: HashSet<building::Id>) {
        self.bank.building_price = target.iter()
            .map(|id| {
                if self.chains.contains(id) {
                    return (*id, 0);
                }

                (*id, self.bank.get_price(PayScope::from_building(id), building::REGISTRY[id].cost.clone()))
            })
            .collect()
    }

    pub fn refresh_wonders_price(&mut self) {
        self.bank.wonder_price = self.wonders.iter()
            .filter_map(|(w, b)| {
                if b.is_some() {
                    None
                } else {
                    Some((*w, self.bank.get_price(PayScope::Wonders, wonder::REGISTRY[w].cost.clone())))
                }
            })
            .collect()
    }
}

impl Default for City {
    fn default() -> Self {
        Self {
            coins: STARTING_CITY_COINS,
            resources: Default::default(),
            score: Default::default(),
            buildings: vec![],
            wonders: vec![],
            tokens: vec![],
            scientific_symbols: vec![],
            chains: vec![],
            bank: Default::default(),
            track: Default::default(),
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

#[derive(Default, Debug)]
pub struct Units {
    pub wonders: Vec<wonder::Id>,
    pub buildings: Vec<building::Id>,
    pub tokens: Vec<token::Id>,
}

#[derive(Default, Debug)]
pub struct RandomUnits {
    pub buildings: HashMap<Age, Vec<building::Id>>,
    pub tokens: Vec<token::Id>,
    pub wonders: Vec<wonder::Id>,
}

#[derive(Debug)]
pub struct Bank {
    pub discard_reward: u8,
    pub building_price: PriceList<building::Id>,
    pub wonder_price: PriceList<wonder::Id>,
    pub resource_price: PriceList<Resource>,
    pub discounts: Vec<Discount>,
}

impl Bank {
    pub fn get_price(&self, scope: PayScope, mut cost: Cost) -> Coins {
        self.discount(scope, &mut cost);

        cost.coins + cost.resources
            .iter()
            .fold(0, |acc, (resource, count)| {
                acc + self.resource_price[resource] * count
            })
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

impl Default for Bank {
    fn default() -> Self {
        Self {
            discard_reward: DEFAULT_DISCARD_REWARD,
            building_price: Default::default(),
            wonder_price: Default::default(),
            resource_price: Resource::ALL
                .iter()
                .map(|r| (*r, DEFAULT_RESOURCE_PRICE))
                .collect(),
            discounts: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Finish {
    pub winner: Nickname,
    pub victory: Victory,
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct Buildings {
    pub layout: Layout,
    pub playable: HashSet<building::Id>,
    pub discarded: Vec<building::Id>,
}

enum ScienceStatus {
    Regular,
    ProgressToken,
    Supremacy,
}

pub(crate) fn get_score(p: Nickname, state: &mut State) -> Score {
    let mut score = Score::default();
    let city = state.cities.get(&p).unwrap();

    for id in city.buildings.iter() {
        let points = building::REGISTRY[id].get_points(state);

        match building::REGISTRY[id].kind {
            Kind::Scientific => score.science += points,
            Kind::Civilian => score.civilian += points,
            Kind::Commercial => score.commercial += points,
            Kind::Guild => score.guilds += points,
            _ => (),
        };
    }

    for (w,b) in city.wonders.iter() {
        if b.is_some() {
            score.wonders += wonder::REGISTRY[&w].get_points(state);
        }
    }

    for id in city.tokens.iter() {
        score.tokens += token::REGISTRY[id].get_points(state);
    }

    score.coins = city.coins / COINS_PER_POINT;
    // points() or get_points()?!
    score.military = city.track.get_points();
    score.total = score.civilian
        + score.science
        + score.commercial
        + score.guilds
        + score.wonders
        + score.tokens
        + score.coins
        + score.military;

    score
}
