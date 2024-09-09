use crate::{*};

#[derive(Default, Debug)]
pub struct Deck {

}

impl Deck {
    pub fn get_returned_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }

    pub fn get_top_line_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }
}

pub type Layout = Vec<Slot>;

pub enum Slot {
    Empty,
    FaceDown,
    FaceDownGuild,
    FaceUp(building::Id),
}