use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::{
    building::{self},
    deck::Layout,
    military::Track,
    player::Finisher,
    prelude::*,
    token,
    wonder
};

#[derive(Default, Debug)]
pub struct State {
    pub age: Age,
    pub phase: Phase,
    pub players: Players,
    pub cities: HashMap<Nickname, City>,
    pub tokens: Vec<Option<token::Id>>,
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

        let winner = match finisher {
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

    pub fn pay(&mut self, scope: PayScope, mut cost: Cost) -> Result<(), Error> {
        let cost_coins = cost.coins;

        cost.resources.iter_mut().
            for_each(|(r, count)| {
                *count = count.saturating_sub(self.me().resources[r]);
            });

        let price = self.me().bank.get_price(scope, cost);

        if price > self.me().coins {
            return Err(Error::NotEnoughCoins);
        }

        self.me_mut().coins -= price;

        if self.enemy().tokens.contains(&token::Id::Economy) {
            self.enemy_mut().coins += price - cost_coins;
        }

        Ok(())
    }

    pub fn move_conflict_pawn(&mut self, power: u8) -> (Coins, bool) {
        let mut fine: Coins = 0;
        let mut supremacy = false;

        if self.enemy().track.pos >= power {
            self.enemy_mut().track.pos -= power;

            return (fine, supremacy);
        }

        self.me_mut().track.pos += power - self.enemy().track.pos;
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
}

impl Default for City {
    fn default() -> Self {
        Self {
            coins: STARTING_CITY_COINS,
            resources: HashMap::from_iter(Resource::ALL.iter().map(|r| (*r, 0u8))),
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

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Score {
    pub civilian: u8,
    pub science: u8,
    pub commercial: u8,
    pub guilds: u8,
    pub wonders: u8,
    pub tokens: u8,
    pub coins: u8,
    pub military: u8,
    pub total: u8,
}

#[derive(Default, Debug)]
pub struct Units {
    // should keep empty slot to show origin order on client
    pub wonders: Vec<Option<wonder::Id>>,
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
pub struct Finish {
    pub winner: Nickname,
    pub victory: Victory,
}

#[derive(Default, Debug)]
pub struct Players {
    pub starts: Nickname,
    pub me: Nickname,
    pub enemy: Nickname,
    pub fallback: Option<Nickname>,
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

    pub fn members(&self) -> Vec<Nickname> {
        vec![self.me, self.enemy]
    }
}

#[derive(Default, Debug)]
pub struct Buildings {
    pub layout: Layout,
    pub playable: HashSet<building::Id>,
    pub discarded: Vec<building::Id>,
}

pub fn after(s: &mut State) {
    if s.phase == Phase::Over {
        return;
    }

    let has_post_effects = !s.post_effects.is_empty();
    let is_over = s.age.is_last()
        && s.phase == Phase::Turn
        && s.deck.is_empty()
        && !has_post_effects;

    if is_over {
        let finisher = Finisher::Winner(resolve_winner(s));
        over(s, finisher, Victory::Civilian);
        return;
    }

    // post effects has own logic to set turn
    // we resolve turn and use this to fallback turn after post effects if needed
    s.resolve_next_turn();

    if !has_post_effects && s.deck.is_empty() && !s.age.is_last() {
        s.age.next();
        s.deck = Deck::new(get_layout(s.age), s.random_units.buildings[&s.age].clone())
    }

    refresh_buildings(s);
    refresh_cities(s);

    if has_post_effects {
        if s.players.fallback.is_none() {
            s.players.fallback = Some(s.players.me);
        }

        s.post_effects.remove(0).apply(s);
    } else if let Some(p) = s.players.fallback {
        // if starts next age, origin turn resolve is priority
        if s.phase != Phase::WhoBeginsTheNextAgeSelection {
            s.phase = Phase::Turn;
            s.players.set_turn(p);
        }

        s.players.fallback = None;
    }

    if s.deck.is_empty() && s.age.is_last() && s.phase == Phase::Turn {
        let finisher = Finisher::Winner(resolve_winner(s));
        over(s, finisher, Victory::Civilian);
    }
}

fn get_score(s: &mut State) -> Score {
    let mut score = Score::default();
    let city = s.me();

    for bid in city.buildings.iter() {
        let points = get_building(bid).get_points(s);

        match get_building(bid).kind {
            building::Kind::Scientific => score.science += points,
            building::Kind::Civilian => score.civilian += points,
            building::Kind::Commercial => score.commercial += points,
            building::Kind::Guild => score.guilds += points,
            _ => (),
        };
    }

    for (wid, b) in city.wonders.iter() {
        if b.is_some() {
            score.wonders += get_wonder(wid).get_points(s);
        }
    }

    for tid in city.tokens.iter() {
        score.tokens += get_token(tid).get_points(s);
    }

    score.coins = city.coins / COINS_PER_POINT;
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

fn get_buildings_price(s: &mut State) -> PriceList<building::Id> {
    let city = s.me();
    s.buildings.playable.iter()
        .map(|id| {
            if city.chains.contains(id) {
                return (*id, 0);
            }

            (*id, city.bank.get_price(PayScope::from_building(id),get_building(id).cost.clone()))
        })
        .collect()
}

fn get_wonders_price(s: &mut State) -> PriceList<wonder::Id> {
    let city = s.me();
    city.wonders.iter()
        .filter_map(|(wid, b)| {
            if b.is_some() {
                None
            } else {
                Some((*wid, city.bank.get_price(PayScope::Wonders, get_wonder(wid).cost.clone())))
            }
        })
        .collect()
}

pub fn resolve_winner(s: &mut State) -> Nickname {
    match s.me().score.total.cmp(&s.enemy().score.total) {
        Ordering::Greater => s.players.me,
        Ordering::Less => s.players.enemy,
        Ordering::Equal => {
            match s.me().score.civilian.cmp(&s.enemy().score.civilian) {
                Ordering::Greater => s.players.me,
                Ordering::Less => s.players.enemy,
                Ordering::Equal => {
                    if s.players.me == s.players.starts {
                        s.players.me
                    } else {
                        s.players.enemy
                    }
                }
            }
        }
    }
}

pub(crate) fn refresh_cities(s: &mut State) {
    let turn = s.players.me;

    vec![s.players.me, s.players.enemy].into_iter()
        .for_each(|p| {
            s.players.set_turn(p);

            let buildings_price = get_buildings_price(s);
            s.me_mut().bank.building_price = buildings_price;

            let wonders_price = get_wonders_price(s);
            s.me_mut().bank.wonder_price = wonders_price;

            let score = get_score(s);
            s.me_mut().score = score;
        });

    s.players.set_turn(turn);
}

pub(crate) fn refresh_buildings(s: &mut State) {
    s.buildings.layout = s.deck.get_public_layout();
    s.buildings.playable = s.deck.get_playable_buildings();
}

pub(crate) fn over(s: &mut State, finisher: Finisher, victory: Victory) {
    if s.phase == Phase::Over {
        return;
    }

    s.phase = Phase::Over;

    let winner = match finisher {
        Finisher::Winner(w) => w,
        Finisher::Loser(l) => {
            if l == s.players.me {
                s.players.enemy
            } else {
                s.players.me
            }
        }
    };

    s.finish = Some(Finish {
        winner,
        victory,
    });
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Age {
    #[default]
    I = 1,
    II,
    III,
}

impl Age {
    pub const ALL: [Age;3] = [Age::I, Age::II, Age::III];

    pub fn next(&mut self) {
        *self = match self {
            Age::I => Age::II,
            Age::II => Age::III,
            Age::III => Age::III,
        }
    }

    pub fn is_last(&self) -> bool {
        *self == Self::III
    }
}

#[derive(Debug, Default,Eq, PartialEq, Copy, Clone)]
pub enum Phase {
    #[default]
    None = 0,
    Over,
    WondersSelection,
    Turn,
    WhoBeginsTheNextAgeSelection,
    BoardTokenSelection,
    RandomTokenSelection,
    DestructBuildingSelection,
    DiscardedBuildingSelection,
    TopLineBuildingSelection,
    ReturnedBuildingSelection,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScientificSymbol {
    Astrology = 1,
    Wheel,
    Sundial,
    Mortar,
    Compass,
    Writing,
    Law,
}

#[derive(Debug, Copy, Clone)]
pub enum Victory {
    Civilian = 1,
    MilitarySupremacy,
    ScienceSupremacy,
    Resign,
    Timeout,
}