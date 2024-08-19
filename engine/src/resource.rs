use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Resource {
    Clay = 1,
    Wood,
    Stone,
    Glass,
    Papyrus,
}

pub type Store = HashMap<Resource, u8>;