use std::collections::HashMap;
use rand;
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
    pub fn apply(self, s: &mut State) -> Result<(), Error> {
        // match self {
        //     Self::Prepare{} => {
        //
        //     },
        //
        //     _ => Ok(())
        // }
        Ok(())
    }
}

struct Prepare {
    p1: Nickname,
    p2: Nickname,
    wonders: Vec<wonder::Id>,
    board_tokens: Vec<token::Id>,
    random_tokens: Vec<token::Id>,
    // buildings: HashMap<Age, Vec<building::Id>>
}

impl Prepare {
    pub fn new(mut p1: Nickname, mut p2: Nickname) -> Self {
        if rand::random() {
            std::mem::swap(&mut p1, &mut p2);
        }

        let mut wonders = wonder::REGISTRY
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<wonder::Id>>();

        let mut board_tokens = token::REGISTRY
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<token::Id>>();

        let mut random_tokens = token::REGISTRY
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<token::Id>>();

        Self {
            p1,
            p2,
            wonders,
            board_tokens,
            random_tokens,
        }
    }
}