use crate::*;

#[derive(Debug)]
pub struct Discount {
    pub scope: Scope,
    pub resources: Vec<Resource>,
    pub count: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum Scope {
    Global,
    Civilian,
    Wonders,
}