use std::cmp::{max, min};
use std::fmt::{Debug};
use crate::{
    prelude::*,
    building,
    token,
    state,
};
use crate::player::Finisher;

#[derive(Debug)]
pub enum Effect {
    Chain(building::Id),
    Coins(Coins),
    CoinsFor(Bonus, Coins),
    DestructBuilding(building::Kind),
    DiscardRewardAdjuster,
    Discounter {
        scope: PayScope,
        resources: Vec<Resource>,
        count: u8,
    },
    Fine(Coins),
    FixedResourcePrice(Vec<Resource>),
    Guild(Bonus, Coins, Points),
    Mathematics,
    Military(u8, bool),
    PickBoardToken,
    PickDiscardedBuilding,
    PickRandomToken,
    PickReturnedBuildings,
    PickTopLineBuilding,
    PlayAgain,
    Points(u8),
    Resource(Resource, u8),
    Science(ScientificSymbol),
}

impl Effect {
    pub fn apply(&self, s: &mut State) {
        match *self {
            Self::Chain(building) => {
                s.me_mut().chains.push(building);
            }

            Self::Coins(coins) => {
                s.me_mut().coins += coins;
            }

            Self::CoinsFor(bonus, coins) => {
                s.me_mut().coins += s.me().bonus_rate(bonus) * coins;
            }

            Self::DestructBuilding(kind) => {
                let buildings = building::filter_by_kind(&s.enemy_mut().buildings, kind);

                if buildings.is_empty() {
                    return;
                }

                s.post_effects.push(PostEffect::DestructBuilding(s.players.me, buildings));
            }

            Self::DiscardRewardAdjuster => {
                s.me_mut().bank.discard_reward += 1;
            }

            Self::Discounter { scope, ref resources, count } => {
                s.me_mut().bank.discounts.push(Discount {
                    scope,
                    resources: resources.clone(),
                    count,
                });
            }

            Self::Fine(coins) => {
                s.enemy_mut().coins -= min(coins, s.enemy().coins);
            }

            Self::FixedResourcePrice(ref resources) => {
                resources.iter()
                    .for_each(|resource| {
                        *s.me_mut().bank.resource_price.get_mut(resource).unwrap() = FIXED_RESOURCE_PRICE;
                    });
            }

            Self::Guild(bonus, coins, ..) => {
                s.me_mut().coins += get_guild_rate(s, bonus) * coins;
            }

            Self::Military(power, use_strategy_token) => {
                let mut power = power;

                if use_strategy_token && s.me().progress_tokens.contains(&token::Id::Strategy) {
                    power += 1;
                }

                let (fine, supremacy) = s.move_conflict_pawn(power);

                if fine > 0 {
                    s.enemy_mut().coins -= min(fine, s.enemy_mut().coins);
                }

                if supremacy {
                    state::over(s, Finisher::Winner(s.players.me), Victory::MilitarySupremacy);
                }
            }

            Self::PickBoardToken => {
                let tokens = s.progress_tokens.iter().flatten().cloned().collect::<Vec<_>>();

                if !tokens.is_empty() {
                    s.post_effects.push(PostEffect::PickBoardToken(s.players.me, tokens));
                }
            }

            Self::PickDiscardedBuilding => {
                if !s.buildings.discarded.is_empty() {
                    s.post_effects.push(PostEffect::PickDiscardedBuilding(s.players.me, s.buildings.discarded.clone()));
                }
            }

            Self::PickRandomToken => {
                if !s.random_units.tokens.is_empty() {
                    s.post_effects.push(PostEffect::PickRandomToken(s.players.me, s.random_units.tokens.clone()));
                }
            }

            Self::PickReturnedBuildings => {
                let returned_buildings = s.deck.get_returned_buildings();

                if !returned_buildings.is_empty() {
                    s.post_effects.push(PostEffect::PickReturnedBuildings(s.players.me, returned_buildings));
                }
            }

            Self::PickTopLineBuilding => {
                let top_line_buildings = s.deck.get_top_line_buildings();

                if !top_line_buildings.is_empty() {
                    s.post_effects.push(PostEffect::PickTopLineBuilding(s.players.me, top_line_buildings));
                }
            }

            Self::PlayAgain => {
                s.play_again = true;
            }

            Self::Resource(r, count) => {
                *s.me_mut().resources.get_mut(&r).unwrap() += count;

                if !s.enemy().bank.has_fixed_resource_price(&r) {
                    *s.enemy_mut().bank.resource_price.get_mut(&r).unwrap() = DEFAULT_RESOURCE_PRICE + s.me().resources[&r];
                }
            }

            Self::Science(symbol) => {
                let pos = s.me()
                    .scientific_symbols
                    .iter()
                    .position(|(s, _)| *s == symbol);

                if let Some(v) = pos {
                    s.me_mut().scientific_symbols[v].1 += 1;

                    if s.me().scientific_symbols[v].1 == SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN {
                        Effect::PickBoardToken.apply(s);
                    }
                } else {
                    s.me_mut().scientific_symbols.push((symbol, 1));
                }

                if s.me().scientific_symbols.len() == DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY as usize {
                    state::over(s, Finisher::Winner(s.players.me), Victory::ScienceSupremacy);
                }
            }

            _ => (),
        };
    }

    pub fn rollback(&self, s: &mut State) {
        match *self {
            Effect::Resource(resource, count) => {
                let current = s.me().resources[&resource];
                *s.me_mut().resources.get_mut(&resource).unwrap() = min(current - count, 0);

                if !s.me().bank.has_fixed_resource_price(&resource) {
                    *s.me_mut().bank.resource_price.get_mut(&resource).unwrap() = DEFAULT_RESOURCE_PRICE + s.enemy().resources[&resource];
                }
            }
            _ => (),
        }
    }

    pub fn get_points(&self, s: &State) -> u8 {
        match *self {
            Self::Guild(bonus, _, points) => {
                get_guild_rate(s, bonus) * points
            }

            Self::Mathematics => {
                s.me().progress_tokens.len() as u8 * 3
            }

            Self::Points(count) => {
                count
            }

            _ => 0
        }
    }
}

#[derive(Debug)]
pub enum PostEffect {
    DestructBuilding(Nickname, Vec<building::Id>),
    PickBoardToken(Nickname, Vec<token::Id>),
    PickDiscardedBuilding(Nickname, Vec<building::Id>),
    PickRandomToken(Nickname, Vec<token::Id>),
    PickReturnedBuildings(Nickname, Vec<building::Id>),
    PickTopLineBuilding(Nickname, Vec<building::Id>),
}

impl PostEffect {
    pub fn apply(self, s: &mut State) {
        match self {
            Self::DestructBuilding(actor, buildings) => {
                s.phase = Phase::DestructBuildingSelection;
                s.players.set_turn(actor);
                s.interactive_units.buildings = buildings;
            }

            Self::PickBoardToken(actor, tokens) => {
                s.phase = Phase::BoardTokenSelection;
                s.players.set_turn(actor);
                s.interactive_units.tokens = tokens;
            }

            Self::PickDiscardedBuilding(actor, buildings) => {
                s.phase = Phase::DiscardedBuildingSelection;
                s.players.set_turn(actor);
                s.interactive_units.buildings = buildings;
            }

            Self::PickRandomToken(actor, tokens) => {
                s.phase = Phase::RandomTokenSelection;
                s.players.set_turn(actor);
                s.interactive_units.tokens = tokens;
            }

            Self::PickReturnedBuildings(actor, buildings) => {
                s.phase = Phase::ReturnedBuildingSelection;
                s.players.set_turn(actor);
                s.interactive_units.buildings = buildings;
            }

            Self::PickTopLineBuilding(actor, buildings) => {
                s.phase = Phase::TopLineBuildingSelection;
                s.players.set_turn(actor);
                s.interactive_units.buildings = buildings;
            }
        }
    }
}

fn get_guild_rate(s: &State, b: Bonus) -> u8 {
    max(s.me().bonus_rate(b), s.enemy().bonus_rate(b))
}
