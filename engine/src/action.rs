use std::collections::HashMap;
use rand::prelude::{*};
use crate::{*};
use crate::player::Finisher;
use crate::state::{City, Players, RandomUnits};


pub enum Action {
    Prepare(Prepare),
    // resign, timeout. (loser, reason)
    Resign(Nickname),
    SelectWhoBeginsTheNextAge(Nickname),
    ConstructWonder(wonder::Id, building::Id),
    ConstructBuilding(building::Id),
    DiscardBuilding(building::Id),
    DestructBuilding(building::Id),
    PickWonder(wonder::Id),
    PickBoardToken(token::Id),
    PickRandomToken(token::Id),
    PickTopLineCard(building::Id),
    PickDiscardedCard(building::Id),
    // pick, give
    PickReturnedCards(building::Id, building::Id),
  }

impl Action {
    pub fn apply(self, s: &mut State) -> Result<(), Error> {
        match self {
            Self::Prepare(v) => {
                if s.phase != Phase::None {
                    return Err(Error::ActionNotAllowed);
                }

                s.age = Age::I;
                s.phase = Phase::WondersSelection;
                s.players = Players{
                    starts: v.p1,
                    me: v.p1,
                    enemy: v.p1,
                };
                s.cities = HashMap::from([
                    (v.p1, City::default()),
                    (v.p2, City::default()),
                ]);
                s.random_units = RandomUnits{
                    buildings: v.buildings,
                    tokens: v.random_tokens,
                    wonders: v.wonders,
                };
                s.interactive_units.wonders = s.random_units.wonders
                    .iter()
                    .take(WONDER_SELECTION_POOL_SIZE)
                    .copied()
                    .collect();

                s.deck = Deck::new(s.age, s.random_units.buildings[&s.age].clone());
            }

            Self::Resign(actor) => {
                s.over(Finisher::Loser(actor), Victory::Resign);
            }

            Self::SelectWhoBeginsTheNextAge(p) => {
                if s.phase != Phase::WhoBeginsTheNextAgeSelection {
                    return Err(Error::ActionNotAllowed);
                }

                s.players.set_turn(p);
                s.phase = Phase::Turn;
            }

            Self::ConstructWonder(wonder, building) => {
                if s.phase != Phase::Turn {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.buildings.playable.contains(&building) {
                    return Err(Error::ActionNotAllowed);
                }

                // @TODO переписать в один иетератор с маппингом ошибки
                let free_wonder = s.me().wonders
                    .iter()
                    .find(|(w, b)| w == &wonder && b.is_none());

                if free_wonder.is_none() {
                    return Err(Error::ActionNotAllowed);
                }

                // @TODO подумать над совместить cost и scope в enum UnitCost(Global, Wonder, Civilian)
                s.pay(PayScope::Wonders, wonder::REGISTRY.get(&wonder).unwrap().cost.clone())?;



            }

            _ => return Ok(())
        }
        Ok(())
    }
}

pub struct Prepare {
    p1: Nickname,
    p2: Nickname,
    wonders: Vec<wonder::Id>,
    board_tokens: Vec<token::Id>,
    random_tokens: Vec<token::Id>,
    buildings: HashMap<Age, Vec<building::Id>>
}

impl Prepare {
    pub fn new(mut p1: Nickname, mut p2: Nickname, o: Options) -> Self {
        if random() {
            std::mem::swap(&mut p1, &mut p2);
        }

        let (board_tokens, random_tokens) = Self::get_random_tokens();

        Self {
            p1,
            p2,
            wonders: Self::get_random_wonders(&o),
            board_tokens,
            random_tokens,
            buildings: Self::get_random_buildings(),
        }
    }

    pub fn get_random_wonders(o: &Options) -> Vec<wonder::Id> {
        wonder::REGISTRY
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| {
                o.with_promo_wonders || !wonder::Id::PROMO.contains(id)
            })
            .choose_multiple(&mut thread_rng(), WONDER_SELECTION_POOL_SIZE * 2)
    }

    pub fn get_random_tokens() -> (Vec<token::Id>, Vec<token::Id>) {
        let tokens = token::REGISTRY
            .iter()
            .map(|(id, _)| *id)
            .choose_multiple(&mut thread_rng(), token::REGISTRY.len());

        (
            tokens.iter().take(STARTING_TOKENS_COUNT).copied().collect(),
            tokens.iter().skip(STARTING_TOKENS_COUNT).take(RANDOM_TOKENS_COUNT).copied().collect()
        )
    }

    pub fn get_random_buildings() -> HashMap<Age, Vec<building::Id>> {
        let mut buildings: HashMap<Age, Vec<building::Id>> = Default::default();

        for age in Age::ALL {
            let mut shuffled = Self::get_shuffle_buildings(age);
            match age {
                Age::III => {
                    let guilds = Self::get_shuffle_guilds();
                    shuffled.iter().take(DECK_LIMIT - GUILDS_LIMIT).collect::<Vec<_>>().extend(&guilds);

                    buildings.insert(
                        age,
                        shuffled.into_iter().choose_multiple(&mut thread_rng(), DECK_LIMIT + GUILDS_LIMIT),
                    );
                }
                _ => {
                    buildings.insert(age, shuffled.into_iter().take(DECK_LIMIT).collect());
                }
            };
        }

        buildings
    }

    fn get_shuffle_buildings(age: Age) -> Vec<building::Id> {
        building::REGISTRY
            .iter()
            .filter(|(_, b)| b.age == age && b.kind != building::Kind::Guild)
            .map(|(id, _)| *id)
            .choose_multiple(&mut thread_rng(), building::REGISTRY.len())
    }

    fn get_shuffle_guilds() -> Vec<building::Id> {
        building::REGISTRY
            .iter()
            .filter(|(_, b)| b.kind == building::Kind::Guild)
            .map(|(id, _)| *id)
            .choose_multiple(&mut thread_rng(), GUILDS_LIMIT)
    }
}