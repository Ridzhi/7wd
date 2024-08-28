use std::collections::HashMap;
use std::sync::LazyLock;
use crate::{Effects, BaseUnit as BaseUnit};
use crate::economy::{Cost, Resource};

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
    pub effects: Effects,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Effects {
        &self.effects
    }
}

pub static REGISTRY: LazyLock<HashMap<Id, Unit>> = LazyLock::new(|| {
    vec![
        Unit{
            id: Id::TheAppianWay,
            cost: Cost{
                coins: 0,
                resources: HashMap::from([
                    (Resource::Papyrus, 1),
                    (Resource::Clay, 2),
                    (Resource::Stone, 2),
                ]),
            },
            effects: vec![

            ]
        },
    ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
});