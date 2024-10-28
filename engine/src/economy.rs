use std::collections::HashMap;
use std::iter::Iterator;
use crate::building;
use crate::prelude::get_building;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PayScope {
    Global,
    Civilian,
    Wonders,
}

impl PayScope {
    pub fn from_building(id: &building::Id) -> Self {
        if get_building(id).kind == building::Kind::Civilian {
            Self::Civilian
        } else {
            Self::Global
        }
    }
}

#[derive(Debug)]
pub struct Discount {
    pub scope: PayScope,
    pub resources: Vec<Resource>,
    pub count: u8,
}

impl Discount {
    pub fn apply(&self, cost: &mut Cost, priority: &[Resource]) {
        let mut reserve = self.count;

        priority.iter()
            .for_each(|resource| {
                if !self.resources.contains(resource) || reserve == 0 {
                    return;
                }

                if let Some(count) = cost.resources.get_mut(resource) {
                    let n = if *count < reserve {
                        *count
                    } else {
                        reserve
                    };

                    *count -= n;
                    reserve -= n
                }
            });
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Resource {
    Clay = 1,
    Wood,
    Stone,
    Glass,
    Papyrus,
}

impl Resource {
    pub const ALL: [Self;5] = [
        Self::Clay,
        Self::Wood,
        Self::Stone,
        Self::Glass,
        Self::Papyrus,
    ];

    pub const RAW_MATERIALS: [Self;3] = [
        Self::Clay,
        Self::Wood,
        Self::Stone,
    ];

    pub const MANUFACTURED_GOODS: [Self; 2] = [
        Self::Glass,
        Self::Papyrus,
    ];
}

pub type Coins = u8;

pub type PriceList<T> = HashMap<T, Coins>;

pub type Resources = HashMap<Resource, u8>;

#[derive(Default, Clone)]
pub struct Cost {
    pub coins: Coins,
    pub resources: Resources,
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

pub type Points = u8;