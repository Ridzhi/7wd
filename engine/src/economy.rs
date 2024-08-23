use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PayScope {
    Global,
    Civilian,
    Wonders,
}

#[derive(Debug)]
pub struct Discount {
    pub scope: PayScope,
    pub resources: Vec<Resource>,
    pub count: u8,
}

impl Discount {
    pub fn apply(&self, cost: &mut Cost, priority: &Vec<Resource>) {
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

pub type Store<T> = HashMap<T, u8>;
pub type PriceList<T> = Store<T>;

pub type Resources = Store<Resource>;

#[derive(Default)]
pub struct Cost {
    pub coins: u8,
    pub resources: Resources,
}

