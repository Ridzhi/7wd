mod effect;
mod state;
mod player;
mod building;
mod wonder;
mod token;
mod economy;
mod military;
mod deck;
mod action;

pub use self::{
    economy::{Resource, Resources, Coins, Cost, PayScope},
    effect::{Effect, PostEffect},
    player::Nickname,
    state::State,
    deck::Deck,
    action::Action,
};

pub const DEFAULT_RESOURCE_PRICE: u8 = 2;
pub const DEFAULT_DISCARD_REWARD: u8 = 2;
pub const STARTING_CITY_COINS: u8 = 7;
pub const STARTING_TOKENS_COUNT: u8 = 5;
pub const RANDOM_TOKENS_COUNT: u8 = 5;
pub const WONDER_SELECTION_POOL_SIZE: u8 = 4;
pub const WONDERS_CONSTRUCT_LIMIT: u8 = 7;
pub const DECK_LIMIT: u8 = 20;
pub const GUILDS_LIMIT: u8 = 3;
pub const COINS_PER_POINT: u8 = 3;
pub const FIXED_RESOURCE_PRICE: u8 = 1;
pub const SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN: u8 = 2;
pub const DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY: u8 = 6;

#[derive(Debug, Default, Copy, Clone)]
pub enum Age {
    #[default]
    I = 1,
    II,
    III,
}

#[derive(Debug, Default,Eq, PartialEq, Copy, Clone)]
pub enum Phase {
    Over = 1,
    #[default]
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

#[derive(Debug, Copy, Clone)]
pub enum Bonus {
    Resources = 1,
    RawMaterials,
    ManufacturedGoods,
    Military,
    Commercial,
    Civilian,
    Science,
    Wonder,
    Coin,
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

pub trait BaseUnit {
    fn effects(&self) -> &Vec<Effect>;

    fn construct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.apply(s)
        }
    }

    fn destruct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.rollback(s)
        }
    }

    fn points(&self, s: &mut State) -> u8 {
        let mut sum: u8 = 0;

        for effect in self.effects() {
            sum += effect.get_points(s)
        }

        sum
    }
}

pub type Points = u8;

pub enum Error {
    ActionNotAllowed,
}