use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};
use std::sync::{LazyLock, OnceLock};
use crate::{
    Age,
    Cost,
    Effect,
    Store,
    Unit as BaseUnit,
};


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    LumberYard = 100,
    LoggingCamp,
    ClayPool,
    ClayPit,
    Quarry,
    StonePit,
    GlassWorks,
    Press,
    GuardTower,
    Workshop,
    Apothecary,
    StoneReserve,
    ClayReserve,
    WoodReserve,
    Stable,
    Garrison,
    Palisade,
    Scriptorium,
    Pharmacist,
    Theater,
    Altar,
    Baths,
    Tavern,

    SawMill = 200,
    BrickYard,
    ShelfQuarry,
    GlassBlower,
    DryingRoom,
    Walls,
    Forum,
    Caravansery,
    CustomHouse,
    CourtHouse,
    HorseBreeders,
    Barracks,
    ArcheryRange,
    ParadeGround,
    Library,
    Dispensary,
    School,
    Laboratory,
    Statue,
    Temple,
    Aqueduct,
    Rostrum,
    Brewery,

    Arsenal = 300,
    Pretorium,
    Academy,
    Study,
    ChamberOfCommerce,
    Port,
    Armory,
    Palace,
    TownHall,
    Obelisk,
    Fortifications,
    SiegeWorkshop,
    Circus,
    University,
    Observatory,
    Gardens,
    Pantheon,
    Senate,
    Lighthouse,
    Arena,

    MerchantsGuild = 400,
    ShipOwnersGuild,
    BuildersGuild,
    MagistratesGuild,
    ScientistsGuild,
    MoneyLendersGuild,
    TacticiansGuild,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Kind {
    RawMaterials = 1,
    ManufacturedGoods,
    Military,
    Scientific,
    Civilian,
    Commercial,
    Guild,
}

pub struct Unit {
    pub id: Id,
    pub age: Age,
    pub kind: Kind,
    pub cost: Cost,
    pub effects: Vec<Effect>,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

pub fn count_by_kind(source: &Vec<Id>, kind: Kind) -> u8 {
    source
        .iter()
        .filter(
            |id|
            REGISTRY
                .get(id)
                .unwrap().kind == kind
        )
        .map(|&id| id)
        .collect::<Vec<_>>().len() as u8
}

pub static REGISTRY: LazyLock<HashMap<Id, Unit>> = LazyLock::new(|| {
    vec![
        Unit {
            id: Id::LumberYard,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 0,
                resources: Store::new(),
            },
            effects: vec![],
        }
    ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_, _>>()
});
