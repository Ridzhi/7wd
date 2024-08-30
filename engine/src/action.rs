use rand::prelude::{*};
use crate::{*};
// use crate::action::Id::Prepare;

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
    pub fn new(mut p1: Nickname, mut p2: Nickname, o: Options) -> Self {
        if random() {
            std::mem::swap(&mut p1, &mut p2);
        }

        let (board_tokens, random_tokens) = Self::tokens();

        Self {
            p1,
            p2,
            wonders: Self::wonders(&o),
            board_tokens,
            random_tokens,
        }
    }

    fn wonders(o: &Options) -> Vec<wonder::Id> {
        wonder::REGISTRY
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| {
                o.with_promo_wonders || !wonder::Id::PROMO.contains(id)
            })
            .choose_multiple(&mut thread_rng(), WONDER_SELECTION_POOL_SIZE as usize * 2)
    }

    fn tokens() -> (Vec<token::Id>, Vec<token::Id>) {
        let tokens = token::REGISTRY
            .iter()
            .map(|(id, _)| *id)
            .choose_multiple(&mut thread_rng(), token::REGISTRY.len());

        (
            tokens.iter().take(STARTING_TOKENS_COUNT).copied().collect(),
            tokens.iter().skip(STARTING_TOKENS_COUNT).take(RANDOM_TOKENS_COUNT).copied().collect()
        )
    }
}