use crate::{Effects, Unit as BaseUnit};
use crate::effect::Effect;

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
    pub effects: Effects,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Effects {
        &self.effects
    }
}