use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum DiscountScope {
    Global,
    Civilian,
    Wonders,
}

#[derive(Default, Debug)]
pub struct Discounter {
    pub discounts: Vec<Discount>,
}

impl Discounter {
    // pub fn discount(&self, scope: Scope)
}

#[derive(Debug)]
pub struct Discount {
    pub scope: DiscountScope,
    pub resources: Vec<Resource>,
    pub count: u8,
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

pub struct Cost {
    pub coins: u8,
    pub resources: Resources,
}

