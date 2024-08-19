mod effect;
mod state;
mod resource;
mod player;
mod building;

pub use self::{
    state::{State},
    resource::{Store},
    player::{Nickname},
    effect::{Effect}
};

#[derive(Debug, Default, Copy, Clone)]
pub enum Age {
    #[default]
    I = 1,
    II,
    III,
}

#[derive(Debug, Default, Copy, Clone)]
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

pub struct Cost {
    pub coins: u8,
    pub resources: Store,
}

pub enum DiscountContext {
    Global,
    Civilian,
    Wonders,
}

pub trait Unit {
    fn effects(&self) -> Vec<Effect>;

    fn construct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.apply(s)
        }
    }

    fn destruct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.discard(s)
        }
    }

    fn points(&self, s: &State) -> u8 {
        let mut sum: u8 = 0;

        for effect in self.effects() {
            sum += effect.points(s)
        }

        sum
    }
}
