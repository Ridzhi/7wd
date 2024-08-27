use derivative::Derivative;
use crate::{Effects, Unit as BaseUnit};

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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Unit {
    pub id: Id,
    #[derivative(Debug="ignore")]
    pub effects: Effects,
}

impl BaseUnit for Unit {
    fn effects(&self) -> &Effects {
        &self.effects
    }
}