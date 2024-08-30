use crate::{*};

pub enum Id {
    Prepare = 1,
    Over,
    SelectWhoBeginsTheNextAge,
    ConstructWonder,
    ConstructBuilding,
    DiscardBuilding,
    DestructBuilding,
    PickWonder,
    PickBoardToken,
    PickRandomToken,
    PickTopLineCard,
    PickDiscardedCard,
    PickReturnedCards,
}

pub enum Action {
    Prepare(Prepare),
}

impl Action {
    pub fn apply(&self, s: &mut State) -> Result<(), Error> {
        Ok(())
    }
}


struct Prepare {

}
