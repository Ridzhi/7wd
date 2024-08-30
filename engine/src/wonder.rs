use std::collections::HashMap;
use std::sync::LazyLock;
use crate::{*};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    TheAppianWay = 1,
    CircusMaximus,
    TheColossus,
    TheGreatLibrary,
    TheGreatLighthouse,
    TheHangingGardens,
    TheMausoleum,
    Piraeus,
    ThePyramids,
    TheSphinx,
    TheStatueOfZeus,
    TheTempleOfArtemis,
    Messe,
    StatueOfLiberty,
}

pub struct Unit {
    pub id: Id,
    pub cost: Cost,
    pub effects: Vec<Effect>,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

pub static REGISTRY: LazyLock<HashMap<Id, Unit>> = LazyLock::new(|| {
    vec![
        Unit{
            id: Id::TheAppianWay,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Clay, 2),
                    (Resource::Stone, 2),
                ])
            },
            effects: vec![
                Effect::Coins(3),
                Effect::Fine(3),
                Effect::PlayAgain,
                Effect::Points(3),
            ],
        },
        Unit{
            id: Id::CircusMaximus,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 1),
                    (Resource::Wood, 1),
                    (Resource::Stone, 2),
                ])
            },
            effects: vec![
                Effect::DestructBuilding(building::Kind::ManufacturedGoods),
                Effect::Military(1, false),
                Effect::Points(3),
            ],
        },
        Unit{
            id: Id::TheColossus,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 1),
                    (Resource::Clay, 3),
                ])
            },
            effects: vec![
                Effect::Military(2, false),
                Effect::Points(3),
            ],
        },
        Unit{
            id: Id::TheGreatLibrary,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Glass, 1),
                    (Resource::Wood, 3),
                ])
            },
            effects: vec![
                Effect::PickRandomToken,
                Effect::Points(4),
            ],
        },
        Unit{
            id: Id::TheGreatLighthouse,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 2),
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                ])
            },
            effects: vec![
                Effect::Discounter {
                    scope: PayScope::Global,
                    resources: Resource::RAW_MATERIALS.to_vec(),
                    count: 1,
                },
                Effect::Points(4),
            ],
        },
        Unit{
            id: Id::TheHangingGardens,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Glass, 1),
                    (Resource::Wood, 2),
                ])
            },
            effects: vec![
                Effect::Coins(6),
                Effect::PlayAgain,
                Effect::Points(3),
            ],
        },
        Unit{
            id: Id::TheMausoleum,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Glass, 2),
                    (Resource::Clay, 2),
                ])
            },
            effects: vec![
                Effect::PickDiscardedBuilding,
                Effect::Points(2),
            ],
        },
        Unit{
            id: Id::Piraeus,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                    (Resource::Wood, 2),
                ])
            },
            effects: vec![
                Effect::Discounter {
                    scope: PayScope::Global,
                    resources: Resource::MANUFACTURED_GOODS.to_vec(),
                    count: 1,
                },
                Effect::PlayAgain,
                Effect::Points(2),
            ],
        },
        Unit{
            id: Id::ThePyramids,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Stone, 3),
                ])
            },
            effects: vec![
                Effect::Points(9),
            ]
        },
        Unit{
            id: Id::TheSphinx,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 2),
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                ])
            },
            effects: vec![
                Effect::PlayAgain,
                Effect::Points(6),
            ]
        },
        Unit{
            id: Id::TheStatueOfZeus,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 2),
                    (Resource::Clay, 1),
                    (Resource::Wood, 1),
                    (Resource::Stone, 1),
                ])
            },
            effects: vec![
                Effect::DestructBuilding(building::Kind::RawMaterials),
                Effect::Military(1, false),
                Effect::Points(3),
            ],
        },
        Unit{
            id: Id::TheTempleOfArtemis,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Papyrus, 1),
                    (Resource::Glass, 1),
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                ])
            },
            effects: vec![
                Effect::Coins(12),
                Effect::PlayAgain,
            ]
        },
        Unit{
            id: Id::Messe,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                    (Resource::Wood, 1),
                    (Resource::Clay, 2),
                ])
            },
            effects: vec![
                Effect::PickTopLineBuilding,
                Effect::Points(2),
            ],
        },
        Unit{
            id: Id::StatueOfLiberty,
            cost: Cost{
                coins: 0,
                resources: Resources::from([
                    (Resource::Glass, 1),
                    (Resource::Papyrus, 1),
                    (Resource::Clay, 1),
                    (Resource::Stone, 1),
                    (Resource::Wood, 1),
                ])
            },
            effects: vec![
                Effect::PickReturnedBuildings,
                Effect::Points(5),
            ],
        },
    ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
});