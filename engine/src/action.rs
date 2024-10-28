use std::collections::HashMap;
use rand::prelude::{*};
use crate::{*};
use crate::player::Finisher;
use crate::state::{City, Players, RandomUnits, refresh_cities, refresh_buildings, after};


pub enum Action {
    Prepare(Setup),
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
    PickTopLineBuilding(building::Id),
    PickDiscardedBuilding(building::Id),
    // pick, give
    PickReturnedBuildings(building::Id, building::Id),
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
                    enemy: v.p2,
                    fallback: None,
                };
                s.cities = HashMap::from([
                    (v.p1, City::default()),
                    (v.p2, City::default()),
                ]);
                s.progress_tokens = v.board_tokens.iter().map(|id| Some(*id)).collect();
                s.random_units = RandomUnits{
                    buildings: v.buildings,
                    tokens: v.random_tokens,
                    wonders: v.wonders,
                };
                s.interactive_units.wonders = s.random_units.wonders
                    .iter()
                    .take(WONDER_SELECTION_POOL_SIZE as usize)
                    .copied()
                    .map(|id| Some(id))
                    .collect();
            }

            Self::Resign(actor) => {
                state::over(s, Finisher::Loser(actor), Victory::Resign);
            }

            Self::SelectWhoBeginsTheNextAge(p) => {
                if s.phase != Phase::WhoBeginsTheNextAgeSelection {
                    return Err(Error::ActionNotAllowed);
                }

                s.players.set_turn(p);
                s.phase = Phase::Turn;
            }

            Self::ConstructWonder(wid, bid) => {
                if s.phase != Phase::Turn {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.buildings.playable.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                let free_wonder = s.me_mut().wonders
                    .iter()
                    .find(|(w, b)| w == &wid && b.is_none());

                if free_wonder.is_none() {
                    return Err(Error::ActionNotAllowed);
                }

                s.pay(PayScope::Wonders, wonder::REGISTRY[&wid].cost.clone())?;
                s.deck.pull_building(&bid);

                s.me_mut().wonders.iter_mut()
                    .for_each(|(w,b)| {
                       if w == &wid {
                           *b = Some(bid)
                       }
                    });

                let total_wonders_constructed = s.me().wonders.iter()
                    .chain(s.enemy().wonders.iter())
                    .filter(|(_, b)| b.is_some())
                    .count();

                if total_wonders_constructed == WONDERS_CONSTRUCT_LIMIT as usize {
                    s.me_mut().wonders
                        .retain(|(_, b)| !b.is_none());

                    s.enemy_mut().wonders
                        .retain(|(_, b)| !b.is_none());
                }

                wonder::REGISTRY[&wid].construct(s);

                if s.me().progress_tokens.contains(&token::Id::Theology) {
                    s.play_again = true;
                }

                after(s);
            }

            Self::ConstructBuilding(bid) => {
                if s.phase != Phase::Turn {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.buildings.playable.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                if s.me().chains.contains(&bid) {
                    if s.me().progress_tokens.contains(&token::Id::Urbanism) {
                        s.me_mut().coins += 4;
                    }
                } else {
                    s.pay(PayScope::from_building(&bid), building::REGISTRY[&bid].cost.clone())?;
                }

                s.me_mut().buildings.push(bid);
                s.deck.pull_building(&bid);

                building::REGISTRY[&bid].construct(s);

                after(s);
            }

            Self::DiscardBuilding(bid) => {
                if s.phase != Phase::Turn {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.buildings.playable.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                s.buildings.discarded.push(bid);
                s.deck.pull_building(&bid);
                s.me_mut().coins += s.me().bank.discard_reward;

                after(s);
            }

            Self::DestructBuilding(bid) => {
                if s.phase != Phase::DestructBuildingSelection {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.interactive_units.buildings.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                s.enemy_mut().buildings.retain(|id| *id != bid);
                building::REGISTRY[&bid].destruct(s);

                after(s);
            }

            Self::PickWonder(wid) => {
                if s.phase != Phase::WondersSelection {
                    return Err(Error::ActionNotAllowed);
                }

                let wi = s.interactive_units.wonders.iter()
                    .enumerate()
                    .find_map(|(ind, val)| {
                       if let Some(id) = val {
                           if *id == wid {
                               return Some(ind);
                           }
                       }

                       return None
                    });

                if let Some(ind) = wi {
                    *s.interactive_units.wonders.get_mut(ind).expect("interactive wonders have ind") = None;
                } else {
                    return Err(Error::ActionNotAllowed);
                }

                s.me_mut().wonders.push((wid, None));

                let picked_count = s.me().wonders.len() + s.enemy().wonders.len();

                // pick scheme
                // [N] - player
                // stage 1: [1][2][2][1]
                // stage 2: [2][1][1][2]
                // after first move 1
                match picked_count {
                    2|6 => (), //  2 wonders in a row
                    _ => s.players.next_turn() // normal flow, next player
                }

                match picked_count as u8 {
                    WONDER_SELECTION_POOL_SIZE => {
                        s.interactive_units.wonders = s.random_units.wonders.iter()
                            .skip(WONDER_SELECTION_POOL_SIZE as usize)
                            .copied()
                            .map(|id| Some(id))
                            .collect();
                    },

                    WONDER_TOTAL_POOL_SIZE => {
                        s.phase = Phase::Turn;
                        s.interactive_units.wonders = vec![];
                        s.deck = Deck::new(get_layout(s.age), s.random_units.buildings[&s.age].clone());
                        refresh_buildings(s);
                        refresh_cities(s);
                    },

                    _ => (),
                }
            }

            Self::PickBoardToken(tid) => {
                return Self::pick_token(s, Phase::BoardTokenSelection, &tid);
            }

            Self::PickRandomToken(tid) => {
                return Self::pick_token(s, Phase::RandomTokenSelection, &tid);
            }

            Self::PickTopLineBuilding(bid) => {
                if s.phase != Phase::TopLineBuildingSelection {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.interactive_units.buildings.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                s.me_mut().buildings.push(bid);
                s.deck.pull_building(&bid);
                building::REGISTRY[&bid].construct(s);

                after(s);
            }

            Self::PickDiscardedBuilding(bid) => {
                if s.phase != Phase::DiscardedBuildingSelection {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.interactive_units.buildings.contains(&bid) {
                    return Err(Error::ActionNotAllowed);
                }

                s.me_mut().buildings.push(bid);
                s.buildings.discarded.retain(|id| *id != bid);
                building::REGISTRY[&bid].construct(s);

                after(s);
            }

            Self::PickReturnedBuildings(pick, give) => {
                if s.phase != Phase::ReturnedBuildingSelection {
                    return Err(Error::ActionNotAllowed);
                }

                if pick == give {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.interactive_units.buildings.contains(&pick) {
                    return Err(Error::ActionNotAllowed);
                }

                if !s.interactive_units.buildings.contains(&give) {
                    return Err(Error::ActionNotAllowed);
                }

                let fallback_turn = s.players.me;

                {
                    s.me_mut().buildings.push(pick);
                    building::REGISTRY[&pick].construct(s);
                }

                {
                    s.players.set_turn(s.players.enemy);
                    s.me_mut().buildings.push(give);
                    building::REGISTRY[&give].construct(s);
                }

                s.players.set_turn(fallback_turn);

                after(s);
            }
        }
        Ok(())
    }

    fn pick_token(s: &mut State, phase: Phase, token: &token::Id) -> Result<(), Error>{
        if s.phase != phase {
            return Err(Error::ActionNotAllowed);
        }

        if !s.interactive_units.tokens.contains(&token) {
            return Err(Error::ActionNotAllowed);
        }

        s.me_mut().progress_tokens.push(token.clone());
        token::REGISTRY[&token].construct(s);
        let t_ind = s.progress_tokens.iter()
            .enumerate()
            .find_map(|(ind, val)| {
                if let Some(id) = val {
                    return if id == token {
                        Some(ind)
                    } else {
                        None
                    }
                }

                return None;
            });

        *s.progress_tokens.get_mut(t_ind.expect("t ind should be")).expect("s.progress_tokens get t by ind") = None;

        Ok(after(s))
    }
}

pub struct Setup {
    pub p1: Nickname,
    pub p2: Nickname,
    pub wonders: Vec<wonder::Id>,
    pub board_tokens: Vec<token::Id>,
    pub random_tokens: Vec<token::Id>,
    pub buildings: HashMap<Age, Vec<building::Id>>
}

impl Setup {
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
            .choose_multiple(&mut thread_rng(), WONDER_TOTAL_POOL_SIZE as usize)
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
            let shuffled = Self::get_shuffle_buildings(age);
            match age {
                Age::III => {
                    let guilds = Self::get_shuffle_guilds();
                    shuffled.iter().take((DECK_LIMIT - GUILDS_LIMIT) as usize).collect::<Vec<_>>().extend(&guilds);

                    buildings.insert(
                        age,
                        shuffled.into_iter().choose_multiple(&mut thread_rng(), (DECK_LIMIT + GUILDS_LIMIT) as usize),
                    );
                }
                _ => {
                    buildings.insert(age, shuffled.into_iter().take(DECK_LIMIT as usize).collect());
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
            .choose_multiple(&mut thread_rng(), GUILDS_LIMIT as usize)
    }
}