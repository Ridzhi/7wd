use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Resource {
    Clay = 1,
    Wood,
    Stone,
    Glass,
    Papyrus,
}

pub type Store = HashMap<Resource, u8>;