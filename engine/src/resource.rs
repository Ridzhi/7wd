use std::collections::HashMap;

pub enum Resource {
    Clay = 1,
    Wood,
    Stone,
    Glass,
    Papyrus,
}

pub type Store = HashMap<Resource, u8>;