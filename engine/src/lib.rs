mod effect;
mod state;
mod player;
mod building;
mod wonder;
mod token;
mod economy;
mod military;
mod deck;

pub use self::{
    economy::{Resource, Resources, Coins},
    effect::{Effect, Effects, PostEffect},
    player::Nickname,
    state::State,
    deck::Deck,
};

pub const COINS_PER_POINT: u8 = 3;
pub const DEFAULT_RESOURCE_PRICE: u8 = 2;
pub const FIXED_RESOURCE_PRICE: u8 = 1;

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

#[derive(Debug, Copy, Clone)]
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

pub trait Unit {
    fn effects(&self) -> &Effects;

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