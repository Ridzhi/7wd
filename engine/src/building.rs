use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};
use std::sync::LazyLock;
use std::vec;
use crate::{*};
use crate::economy::PayScope;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
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

pub fn filter_by_kind(source: &Vec<Id>, kind: Kind) -> Vec<Id> {
    source
        .iter()
        .filter(
            |id|
                REGISTRY
                    .get(id)
                    .unwrap().kind == kind
        )
        .map(|&id| id)
        .collect::<Vec<_>>()
}

pub fn count_by_kind(source: &Vec<Id>, kind: Kind) -> u8 {
    filter_by_kind(source, kind).len() as u8
}

pub static REGISTRY: LazyLock<HashMap<Id, Unit>> = LazyLock::new(|| {
    vec![
        Unit {
            id: Id::LumberYard,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Wood, 1),
            ],
        },
        Unit {
            id: Id::LoggingCamp,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 1,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Wood, 1),
            ],
        },
        Unit {
            id: Id::ClayPool,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Clay, 1),
            ],
        },
        Unit {
            id: Id::ClayPit,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 1,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Clay, 1),
            ],
        },
        Unit {
            id: Id::Quarry,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Stone, 1)
            ],
        },
        Unit {
            id: Id::StonePit,
            age: Age::I,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 1,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Clay, 1)
            ],
        },
        Unit {
            id: Id::GlassWorks,
            age: Age::I,
            kind: Kind::ManufacturedGoods,
            cost: Cost {
                coins: 1,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Glass, 1)
            ],
        },
        Unit {
            id: Id::Press,
            age: Age::I,
            kind: Kind::ManufacturedGoods,
            cost: Cost {
                coins: 1,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Papyrus, 1)
            ],
        },
        Unit {
            id: Id::GuardTower,
            age: Age::I,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Military(1, true),
            ],
        },
        Unit {
            id: Id::Workshop,
            age: Age::I,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(1),
                Effect::Science(ScientificSymbol::Compass),
            ],
        },
        Unit {
            id: Id::Apothecary,
            age: Age::I,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(1),
                Effect::Science(ScientificSymbol::Wheel),
            ],
        },
        Unit {
            id: Id::StoneReserve,
            age: Age::I,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 3,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::FixedResourcePrice(vec![Resource::Stone]),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::ClayReserve,
            age: Age::I,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 3,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::FixedResourcePrice(vec![Resource::Clay]),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::WoodReserve,
            age: Age::I,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 3,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::FixedResourcePrice(vec![Resource::Wood]),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Stable,
            age: Age::I,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 1),
                ]),
            },
            effects: vec![
                Effect::Military(1, true),
                Effect::Chain(Id::HorseBreeders),
            ],
        },
        Unit {
            id: Id::Garrison,
            age: Age::I,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                ]),
            },
            effects: vec![
                Effect::Military(1, true),
                Effect::Chain(Id::Barracks),
            ],
        },
        Unit {
            id: Id::Palisade,
            age: Age::I,
            kind: Kind::Military,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Military(1, true),
                Effect::Chain(Id::Fortifications),
            ],
        },
        Unit {
            id: Id::Scriptorium,
            age: Age::I,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Science(ScientificSymbol::Writing),
                Effect::Chain(Id::Library),
            ],
        },
        Unit {
            id: Id::Pharmacist,
            age: Age::I,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Science(ScientificSymbol::Mortar),
                Effect::Chain(Id::Dispensary),
            ],
        },
        Unit {
            id: Id::Theater,
            age: Age::I,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Points(3),
                Effect::Chain(Id::Statue),
            ],
        },
        Unit {
            id: Id::Altar,
            age: Age::I,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Points(3),
                Effect::Chain(Id::Temple)
            ],
        },
        Unit {
            id: Id::Baths,
            age: Age::I,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::Chain(Id::Aqueduct),
            ],
        },
        Unit {
            id: Id::Tavern,
            age: Age::I,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Coins(4),
                Effect::Chain(Id::Lighthouse),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::SawMill,
            age: Age::II,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Wood, 2),
            ],
        },
        Unit {
            id: Id::BrickYard,
            age: Age::II,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Clay, 2)
            ],
        },
        Unit {
            id: Id::ShelfQuarry,
            age: Age::II,
            kind: Kind::RawMaterials,
            cost: Cost {
                coins: 2,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Stone, 2)
            ],
        },
        Unit {
            id: Id::GlassBlower,
            age: Age::II,
            kind: Kind::ManufacturedGoods,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Glass, 1)
            ],
        },
        Unit {
            id: Id::DryingRoom,
            age: Age::II,
            kind: Kind::ManufacturedGoods,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Resource(Resource::Papyrus, 1)
            ],
        },
        Unit {
            id: Id::Walls,
            age: Age::II,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
            ],
        },
        Unit {
            id: Id::Forum,
            age: Age::II,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 3,
                resources: Resources::from([
                    (Resource::Clay, 1),
                ]),
            },
            effects: vec![
                Effect::Discounter {
                    scope: PayScope::Global,
                    resources: vec![Resource::Glass, Resource::Papyrus],
                    count: 1,
                },
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Caravansery,
            age: Age::II,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 2,
                resources: Resources::from([
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Discounter {
                    scope: PayScope::Global,
                    resources: vec![Resource::Clay, Resource::Wood, Resource::Stone],
                    count: 1,
                },
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::CustomHouse,
            age: Age::II,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 4,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::FixedResourcePrice(vec![Resource::Papyrus, Resource::Glass]),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::CourtHouse,
            age: Age::II,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 2),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(5),
            ],
        },
        Unit {
            id: Id::HorseBreeders,
            age: Age::II,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Wood, 1),
                ]),
            },
            effects: vec![
                Effect::Military(1, true),
            ],
        },
        Unit {
            id: Id::Barracks,
            age: Age::II,
            kind: Kind::Military,
            cost: Cost {
                coins: 3,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Military(1, true),
            ],
        },
        Unit {
            id: Id::ArcheryRange,
            age: Age::II,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
                Effect::Chain(Id::SiegeWorkshop),
            ],
        },
        Unit {
            id: Id::ParadeGround,
            age: Age::II,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
                Effect::Chain(Id::Circus),
            ],
        },
        Unit {
            id: Id::Library,
            age: Age::II,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(2),
                Effect::Science(ScientificSymbol::Writing),
            ],
        },
        Unit {
            id: Id::Dispensary,
            age: Age::II,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Stone, 1),
                ]),
            },
            effects: vec![
                Effect::Points(2),
                Effect::Science(ScientificSymbol::Mortar),
            ],
        },
        Unit {
            id: Id::School,
            age: Age::II,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 1),
                    (Resource::Papyrus, 2),
                ]),
            },
            effects: vec![
                Effect::Points(1),
                Effect::Science(ScientificSymbol::Wheel),
                Effect::Chain(Id::University),
            ],
        },
        Unit {
            id: Id::Laboratory,
            age: Age::II,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 1),
                    (Resource::Glass, 2),
                ]),
            },
            effects: vec![
                Effect::Points(1),
                Effect::Science(ScientificSymbol::Compass),
                Effect::Chain(Id::Observatory),
            ],
        },
        Unit {
            id: Id::Statue,
            age: Age::II,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                ]),
            },
            effects: vec![
                Effect::Points(4),
                Effect::Chain(Id::Gardens),
            ],
        },
        Unit {
            id: Id::Temple,
            age: Age::II,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(4),
                Effect::Chain(Id::Pantheon),
            ],
        },
        Unit {
            id: Id::Aqueduct,
            age: Age::II,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 3),
                ]),
            },
            effects: vec![
                Effect::Points(5),
            ],
        },
        Unit {
            id: Id::Rostrum,
            age: Age::II,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                ]),
            },
            effects: vec![
                Effect::Points(4),
                Effect::Chain(Id::Senate),
            ],
        },
        Unit {
            id: Id::Brewery,
            age: Age::II,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Coins(6),
                Effect::Chain(Id::Arena),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Arsenal,
            age: Age::III,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 3),
                    (Resource::Wood, 2),
                ]),
            },
            effects: vec![
                Effect::Military(3, true),
            ],
        },
        Unit {
            id: Id::Pretorium,
            age: Age::III,
            kind: Kind::Military,
            cost: Cost {
                coins: 8,
                resources: Resources::new(),
            },
            effects: vec![
                Effect::Military(3, true),
            ],
        },
        Unit {
            id: Id::Academy,
            age: Age::III,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                    (Resource::Glass, 2),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::Science(ScientificSymbol::Sundial),
            ],
        },
        Unit {
            id: Id::Study,
            age: Age::III,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 2),
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::Science(ScientificSymbol::Sundial),
            ],
        },
        Unit {
            id: Id::ChamberOfCommerce,
            age: Age::III,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 2),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::CoinsFor(Bonus::ManufacturedGoods, 3),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Port,
            age: Age::III,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 1),
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::CoinsFor(Bonus::RawMaterials, 2),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Armory,
            age: Age::III,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::CoinsFor(Bonus::Military, 1),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Palace,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                    (Resource::Glass, 2),
                ]),
            },
            effects: vec![
                Effect::Points(7),
            ],
        },
        Unit {
            id: Id::TownHall,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 3),
                    (Resource::Wood, 2),
                ]),
            },
            effects: vec![
                Effect::Points(7),
            ],
        },
        Unit {
            id: Id::Obelisk,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(5),
            ],
        },
        Unit {
            id: Id::Fortifications,
            age: Age::III,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Clay, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
            ],
        },
        Unit {
            id: Id::SiegeWorkshop,
            age: Age::III,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 3),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
            ],
        },
        Unit {
            id: Id::Circus,
            age: Age::III,
            kind: Kind::Military,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Stone, 2),
                ]),
            },
            effects: vec![
                Effect::Military(2, true),
            ],
        },
        Unit {
            id: Id::University,
            age: Age::III,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(2),
                Effect::Science(ScientificSymbol::Astrology),
            ],
        },
        Unit {
            id: Id::Observatory,
            age: Age::III,
            kind: Kind::Scientific,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 1),
                    (Resource::Papyrus, 2),
                ]),
            },
            effects: vec![
                Effect::Points(2),
                Effect::Science(ScientificSymbol::Astrology),
            ],
        },
        Unit {
            id: Id::Gardens,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Wood, 2),
                ]),
            },
            effects: vec![
                Effect::Points(6),
            ],
        },
        Unit {
            id: Id::Pantheon,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Wood, 1),
                    (Resource::Papyrus, 2),
                ]),
            },
            effects: vec![
                Effect::Points(6),
            ],
        },
        Unit {
            id: Id::Senate,
            age: Age::III,
            kind: Kind::Civilian,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Stone, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Points(5),
            ],
        },
        Unit {
            id: Id::Lighthouse,
            age: Age::III,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::CoinsFor(Bonus::Commercial, 1),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::Arena,
            age: Age::III,
            kind: Kind::Commercial,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                ]),
            },
            effects: vec![
                Effect::Points(3),
                Effect::CoinsFor(Bonus::Wonder, 2),
                Effect::DiscardRewardAdjuster,
            ],
        },
        Unit {
            id: Id::MerchantsGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Wood, 1),
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Commercial, 1, 1),
            ],
        },
        Unit {
            id: Id::ShipOwnersGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Resources, 1, 1),
            ],
        },
        Unit {
            id: Id::BuildersGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Clay, 1),
                    (Resource::Wood, 1),
                    (Resource::Glass, 1),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Wonder, 0, 2),
            ],
        },
        Unit {
            id: Id::MagistratesGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Wood, 2),
                    (Resource::Clay, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Civilian, 1, 1),
            ],
        },
        Unit {
            id: Id::ScientistsGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 2),
                    (Resource::Wood, 2),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Science, 1, 1),
            ],
        },
        Unit {
            id: Id::MoneyLendersGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Wood, 2),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Coin, 0, 1),
            ],
        },
        Unit {
            id: Id::TacticiansGuild,
            age: Age::III,
            kind: Kind::Guild,
            cost: Cost {
                coins: 0,
                resources: Resources::from([
                    (Resource::Stone, 2),
                    (Resource::Clay, 1),
                    (Resource::Papyrus, 1),
                ]),
            },
            effects: vec![
                Effect::Guild(Bonus::Military, 1, 1),
            ],
        },
    ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_, _>>()
});
