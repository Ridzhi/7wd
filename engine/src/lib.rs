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
    deck::{Deck, get_layout},
    action::{Action, Setup},
};

pub const DEFAULT_RESOURCE_PRICE: u8 = 2;
pub const DEFAULT_DISCARD_REWARD: u8 = 2;
pub const STARTING_CITY_COINS: u8 = 7;
pub const STARTING_TOKENS_COUNT: usize = 5;
pub const RANDOM_TOKENS_COUNT: usize = 5;
pub const WONDER_SELECTION_POOL_SIZE: usize = 4;
pub const WONDERS_CONSTRUCT_LIMIT: usize = 7;
pub const DECK_LIMIT: usize = 20;
pub const GUILDS_LIMIT: usize = 3;
pub const COINS_PER_POINT: u8 = 3;
pub const FIXED_RESOURCE_PRICE: u8 = 1;
pub const SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN: u8 = 2;
pub const DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY: u8 = 6;

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

    fn get_points(&self, s: &State) -> u8 {
        let mut sum: u8 = 0;

        for effect in self.effects() {
            sum += effect.get_points(s)
        }

        sum
    }
}

pub type Points = u8;

#[derive(Debug)]
pub enum Error {
    ActionNotAllowed,
    NotEnoughCoins,
}

#[derive(Default)]
pub struct Options {
    pub with_promo_wonders: bool,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use Action::{*};
    use wonder::Id::{*};
    use token::Id::{*};
    use building::Id::{*};
    use crate::state::Score;

    #[test]
    fn game_11() {
        let actions = vec![
            Prepare(Setup{
                p1: 1,
                p2: 2,
                wonders: vec![
                    TheHangingGardens,
                    TheTempleOfArtemis,
                    TheColossus,
                    Messe,
                    ThePyramids,
                    StatueOfLiberty,
                    TheMausoleum,
                    TheSphinx,
                ],
                board_tokens: vec![
                    Economy,
                    Agriculture,
                    Philosophy,
                    Theology,
                    Law,
                ],
                random_tokens: vec![
                    Urbanism,
                    Strategy,
                    Masonry,
                ],
                buildings: HashMap::from([
                    (
                        Age::I,
                        vec![
                            Palisade,
                            Theater,
                            Tavern,
                            Stable,
                            Altar,
                            Workshop,
                            ClayReserve,
                            GlassWorks,
                            LoggingCamp,
                            LumberYard,
                            Baths,
                            Quarry,
                            ClayPit,
                            ClayPool,
                            Scriptorium,
                            Garrison,
                            StonePit,
                            WoodReserve,
                            Pharmacist,
                            StoneReserve,
                        ],
                    ),
                    (
                        Age::II,
                        vec![
                            Dispensary,
                            CustomHouse,
                            CourtHouse,
                            Caravansery,
                            GlassBlower,
                            BrickYard,
                            School,
                            Laboratory,
                            Aqueduct,
                            ArcheryRange,
                            ParadeGround,
                            Brewery,
                            Statue,
                            HorseBreeders,
                            ShelfQuarry,
                            Library,
                            Walls,
                            SawMill,
                            Barracks,
                            DryingRoom,
                        ],
                    ),
                    (
                        Age::III,
                        vec![
                            Port,
                            Academy,
                            Obelisk,
                            Observatory,
                            Fortifications,
                            Palace,
                            Senate,
                            Armory,
                            MagistratesGuild,
                            MerchantsGuild,
                            SiegeWorkshop,
                            ChamberOfCommerce,
                            Arsenal,
                            Pretorium,
                            Arena,
                            Lighthouse,
                            Gardens,
                            Pantheon,
                            MoneyLendersGuild,
                            TownHall,
                        ],
                    )
                ]),
            }),
            PickWonder(TheTempleOfArtemis),
            PickWonder(TheHangingGardens),
            PickWonder(TheColossus),
            PickWonder(Messe),
            PickWonder(TheSphinx),
            PickWonder(StatueOfLiberty),
            PickWonder(TheMausoleum),
            PickWonder(ThePyramids),
            ConstructBuilding(WoodReserve),
            ConstructBuilding(StoneReserve),
            ConstructBuilding(Scriptorium),
            ConstructBuilding(StonePit),
            ConstructBuilding(Quarry),
            DiscardBuilding(Garrison),
            ConstructBuilding(Pharmacist),
            ConstructBuilding(ClayPool),
            ConstructBuilding(LumberYard),
            ConstructBuilding(Baths),
            DiscardBuilding(ClayPit),
            ConstructBuilding(LoggingCamp),
            ConstructBuilding(GlassWorks),
            ConstructBuilding(Altar),
            ConstructBuilding(Workshop),
            DiscardBuilding(ClayReserve),
            ConstructBuilding(Tavern),
            ConstructBuilding(Stable),
            ConstructBuilding(Theater),
            ConstructBuilding(Palisade),
            SelectWhoBeginsTheNextAge(1),
            ConstructBuilding(DryingRoom),
            ConstructBuilding(SawMill),
            ConstructBuilding(ShelfQuarry),
            DiscardBuilding(ParadeGround),
            ConstructBuilding(BrickYard),
            ConstructBuilding(Barracks),
            ConstructBuilding(Library),
            PickBoardToken(Theology),
            ConstructBuilding(Walls),
            ConstructBuilding(Brewery),
            DiscardBuilding(HorseBreeders),
            ConstructWonder(Messe, Statue),
            PickTopLineBuilding(Dispensary),
            PickBoardToken(Economy),
            ConstructBuilding(Laboratory),
            PickBoardToken(Agriculture),
            ConstructBuilding(ArcheryRange),
            ConstructBuilding(Aqueduct),
            ConstructBuilding(GlassBlower),
            ConstructBuilding(School),
            DiscardBuilding(CourtHouse),
            ConstructBuilding(Caravansery),
            ConstructBuilding(CustomHouse),
            SelectWhoBeginsTheNextAge(1),
            ConstructWonder(TheMausoleum, MoneyLendersGuild),
            PickDiscardedBuilding(ParadeGround),
            ConstructBuilding(Lighthouse),
            ConstructBuilding(ChamberOfCommerce),
            ConstructBuilding(TownHall),
            ConstructWonder(ThePyramids, Gardens),
            ConstructBuilding(Arsenal),
            DiscardBuilding(Pantheon),
            DiscardBuilding(Pretorium),
            ConstructBuilding(MerchantsGuild),
            ConstructWonder(StatueOfLiberty, Senate),
            PickReturnedBuildings(Study, Circus),
            ConstructWonder(TheTempleOfArtemis, Palace),
            ConstructBuilding(Obelisk),
            ConstructBuilding(Arena),
            ConstructBuilding(SiegeWorkshop),
            ConstructBuilding(MagistratesGuild),
            ConstructBuilding(Armory),
            ConstructBuilding(Observatory),
            ConstructBuilding(Fortifications),
            ConstructBuilding(Port),
            ConstructBuilding(Academy),
            PickBoardToken(Philosophy),
        ];
        let s = State::from(actions).expect("its ok");

        // coins 33
        let expected1 = Score{
            civilian: 20,
            science: 13,
            commercial: 6,
            guilds: 0,
            wonders: 9,
            tokens: 11,
            coins: 11,
            military: 0,
            total: 70,
        };

        // coins 19
        let expected2 = Score{
            civilian: 6,
            science: 2,
            commercial: 9,
            guilds: 10,
            wonders: 9,
            tokens: 0,
            coins: 6,
            military: 0,
            total: 42,
        };

        let p1 = s.enemy();
        let p2 = s.me();

        let actual1 = p1.score;
        let actual2 = p2.score;
        // assert_eq!(33, p1.coins);
        // assert_eq!(19, p2.coins);
        assert_eq!(expected1.military, actual1.military);
        // assert_eq!(expected2, actual2);
    }
}