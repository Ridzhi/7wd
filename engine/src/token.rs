use std::collections::HashMap;
use std::sync::LazyLock;
use crate::{BaseUnit, Effect, Resource, ScientificSymbol};
use crate::economy::PayScope;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Id {
    Agriculture = 1,
    Architecture,
    Economy,
    Law,
    Masonry,
    Mathematics,
    Philosophy,
    Strategy,
    Theology,
    Urbanism,
}

#[derive(Debug)]
pub struct Unit {
    pub id: Id,
    pub effects: Vec<Effect>,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

pub static REGISTRY: LazyLock<HashMap<Id, Unit>> = LazyLock::new(|| {
    vec![
        Unit {
            id: Id::Agriculture,
            effects: vec![
                Effect::Coins(6),
                Effect::Points(4),
            ],
        },
        Unit {
            id: Id::Architecture,
            effects: vec![
                Effect::Discounter{
                    scope: PayScope::Wonders,
                    resources: Resource::ALL.to_vec(),
                    count: 2,
                },
            ],
        },
        Unit {
            id: Id::Economy,
            effects: vec![],
        },
        Unit {
            id: Id::Law,
            effects: vec![
                Effect::Science(ScientificSymbol::Law),
            ],
        },
        Unit {
            id: Id::Masonry,
            effects: vec![
                Effect::Discounter{
                    scope: PayScope::Civilian,
                    resources: Resource::ALL.to_vec(),
                    count: 2,
                },
            ],
        },
        Unit {
            id: Id::Mathematics,
            effects: vec![
                Effect::Mathematics,
            ],
        },
        Unit {
            id: Id::Philosophy,
            effects: vec![
                Effect::Points(7),
            ],
        },
        Unit {
            id: Id::Strategy,
            effects: vec![],
        },
        Unit {
            id: Id::Theology,
            effects: vec![],
        },
        Unit {
            id: Id::Urbanism,
            effects: vec![
                Effect::Coins(6),
            ],
        },
    ]
        .into_iter()
        .map(|unit| (unit.id, unit))
        .collect::<HashMap<_,_>>()
});