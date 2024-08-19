use crate::{Effect, Unit as BaseUnit};

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

pub struct Unit {
    pub id: Id,
    pub effects: Vec<Effect>,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}