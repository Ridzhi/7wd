use std::cmp::{max, min};
use std::fmt::{Debug};
use crate::{
    building,
    economy::{Coins, Discount, PayScope},
    state::{City, State, Units},
    *,
};

pub trait Effect {
    fn apply(&self, s: &mut State) {
        ()
    }

    fn rollback(&self, s: &mut State) {
        ()
    }

    fn get_points(&self, s: &mut State) -> u8 {
        0
    }
}

pub type Effects = Vec<Box<dyn Effect + Send + Sync>>;

pub trait PostEffect {
    fn apply(self, s: &mut State);
}

#[derive(Debug)]
pub struct Chain {
    pub building: building::Id,
}

impl Effect for Chain {
    fn apply(&self, s: &mut State) {
        s.me_mut().chains.push(self.building);
    }
}

#[derive(Debug)]
pub struct Reward {
    coins: Coins,
}

impl Effect for Reward {
    fn apply(&self, s: &mut State) {
        s.me_mut().coins += self.coins;
    }
}

#[derive(Debug)]
pub struct RewardFor {
    pub coins: Coins,
    pub bonus: Bonus,
}

impl Effect for RewardFor {
    fn apply(&self, s: &mut State) {
        s.me_mut().coins += s.me_mut().bonus_rate(self.bonus) * self.coins;
    }
}

#[derive(Debug)]
pub struct DestructBuilding {
    pub kind: building::Kind,
}

impl Effect for DestructBuilding {
    fn apply(&self, s: &mut State) {
        let buildings = building::filter_by_kind(&s.enemy_mut().buildings, self.kind);

        if buildings.is_empty() {
            return;
        }

        s.post_effects.push(Box::new(
            PostDestructBuilding {
                actor: s.players.me.clone(),
                buildings,
            }
        ));
    }
}

#[derive(Debug)]
pub struct PostDestructBuilding {
    pub actor: Nickname,
    pub buildings: Vec<building::Id>,
}

impl PostEffect for PostDestructBuilding {
    fn apply(self, s: &mut State) {
        s.phase = Phase::DestructBuildingSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.buildings = self.buildings;
    }
}

#[derive(Debug)]
pub struct DiscardRewardAdjuster;

impl Effect for DiscardRewardAdjuster {
    fn apply(&self, s: &mut State) {
        s.me_mut().bank.discard_reward += 1;
    }
}

#[derive(Debug)]
pub struct Discounter {
    scope: PayScope,
    resources: Vec<Resource>,
    count: u8,
}

impl Effect for Discounter {
    fn apply(&self, s: &mut State) {
        s.me_mut().bank.discounts.push(Discount {
            scope: self.scope,
            resources: self.resources.clone(),
            count: self.count,
        })
    }
}

#[derive(Debug)]
pub struct Fine {
    pub coins: Coins,
}

impl Effect for Fine {
    fn apply(&self, s: &mut State) {
        s.enemy_mut().coins -= min(self.coins, s.enemy_mut().coins);
    }
}

#[derive(Debug)]
pub struct FixedPrice {
    pub resources: Vec<Resource>,
}

impl Effect for FixedPrice {
    fn apply(&self, s: &mut State) {
        self.resources.iter()
            .for_each(|resource| {
                *s.me_mut().bank.resource_price.get_mut(resource).unwrap() = FIXED_RESOURCE_PRICE;
            })
    }
}

#[derive(Debug)]
pub struct Guild {
    pub bonus: Bonus,
    pub points: u8,
    pub coins: Coins,
}

impl Guild {
    fn rate(&self, s: &mut State) -> u8 {
        max(s.me_mut().bonus_rate(self.bonus), s.enemy_mut().bonus_rate(self.bonus))
    }
}

impl Effect for Guild {
    fn apply(&self, s: &mut State) {
        s.me_mut().coins += self.rate(s) * self.coins;
    }

    fn get_points(&self, s: &mut State) -> u8 {
        self.rate(s) * self.points
    }
}

#[derive(Debug)]
pub struct Mathematics;

impl Effect for Mathematics {
    fn get_points(&self, s: &mut State) -> u8 {
        s.me_mut().tokens.len() as u8 * 3
    }
}

#[derive(Debug)]
pub struct Military {
    pub power: u8,
    pub apply_strategy_token: bool,
}

impl Military {
    pub fn for_building(power: u8) -> Self {
        Self {
            power,
            apply_strategy_token: true,
        }
    }

    pub fn for_wonder(power: u8) -> Self {
        Self {
            power,
            apply_strategy_token: false,
        }
    }
}

impl Effect for Military {
    fn apply(&self, s: &mut State) {
        let power = self.power + {
            if self.apply_strategy_token && s.me().tokens.contains(&token::Id::Strategy) {
                1
            } else {
                0
            }
        };

        let (fine, supremacy) = s.move_conflict_pawn(power);

        if fine > 0 {
            s.enemy_mut().coins -= min(fine, s.enemy_mut().coins);
        }

        if supremacy {
            s.over(Victory::MilitarySupremacy, s.players.me.clone())
        }
    }
}

#[derive(Debug)]
pub struct PickBoardToken;

impl Effect for PickBoardToken {
    fn apply(&self, s: &mut State) {
        // чек не пустые

        s.post_effects.push(Box::new(PostPickBoardToken {
            actor: s.players.me.clone(),
        }));
    }
}

#[derive(Debug)]
pub struct PostPickBoardToken {
    pub actor: Nickname,
}

impl PostEffect for PostPickBoardToken {
    fn apply(self, s: &mut State) {
        s.phase = Phase::BoardTokenSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.tokens = s.progress_tokens.clone();
    }
}

#[derive(Debug)]
pub struct PickDiscardedBuilding;

impl Effect for PickDiscardedBuilding {
    fn apply(&self, s: &mut State) {
        if !s.buildings.discarded.is_empty() {
            s.post_effects.push(Box::new(PostPickDiscardedBuilding {
                actor: s.players.me.clone(),
                buildings: s.buildings.discarded.clone(),
            }));
        }
    }
}

#[derive(Debug)]
pub struct PostPickDiscardedBuilding {
    pub actor: Nickname,
    pub buildings: Vec<building::Id>,
}

impl PostEffect for PostPickDiscardedBuilding {
    fn apply(self, s: &mut State) {
        s.phase = Phase::DiscardedBuildingSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.buildings = self.buildings;
    }
}

#[derive(Debug)]
pub struct PickRandomToken;

impl Effect for PickRandomToken {
    fn apply(&self, s: &mut State) {
        if !s.random_units.tokens.is_empty() {
            s.post_effects.push(Box::new(PostPickRandomToken {
                actor: s.players.me.clone(),
                tokens: s.random_units.tokens.clone(),
            }));
        }
    }
}

#[derive(Debug)]
pub struct PostPickRandomToken {
    pub actor: Nickname,
    pub tokens: Vec<token::Id>,
}

impl PostEffect for PostPickRandomToken {
    fn apply(self, s: &mut State) {
        s.phase = Phase::RandomTokenSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.tokens = self.tokens;
    }
}

#[derive(Debug)]
pub struct PickReturnedBuildings;

impl Effect for PickReturnedBuildings {
    fn apply(&self, s: &mut State) {
        let returned_buildings = s.deck.get_returned_buildings();

        if !returned_buildings.is_empty() {
            s.post_effects.push(Box::new(PostPickReturnedBuildings {
                actor: s.players.me.clone(),
                buildings: returned_buildings,
            }));
        }
    }
}

#[derive(Debug)]
pub struct PostPickReturnedBuildings {
    pub actor: Nickname,
    pub buildings: Vec<building::Id>,
}

impl PostEffect for PostPickReturnedBuildings {
    fn apply(self, s: &mut State) {
        s.phase = Phase::ReturnedBuildingSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.buildings = self.buildings;
    }
}

pub struct PickTopLineBuilding;

impl Effect for PickTopLineBuilding {
    fn apply(&self, s: &mut State) {
        let top_line_buildings = s.deck.get_top_line_buildings();

        if !top_line_buildings.is_empty() {
            s.post_effects.push(Box::new(PostPickTopLineBuilding {
                actor: s.players.me.clone(),
                buildings: top_line_buildings,
            }));
        }
    }
}
pub struct PostPickTopLineBuilding {
    pub actor: Nickname,
    pub buildings: Vec<building::Id>,
}

impl PostEffect for PostPickTopLineBuilding {
    fn apply(self, s: &mut State) {
        s.phase = Phase::TopLineBuildingSelection;
        s.players.set_turn(self.actor);
        s.interactive_units.buildings = self.buildings;
    }
}

pub struct PlayAgain;

impl Effect for PlayAgain {
    fn apply(&self, s: &mut State) {
        s.play_again = true;
    }
}

pub struct Points {
    pub count: u8,
}

impl Points {
    pub fn new(count: u8) -> Self {
        Self {
            count
        }
    }
}

impl Effect for Points {
    fn get_points(&self, s: &mut State) -> u8 {
        self.count
    }
}

pub struct ProduceResource {
    pub kind: Resource,
    pub count: u8,
}

impl ProduceResource {
    pub fn new(kind: Resource, count: u8) -> Self {
        Self {
            kind,
            count,
        }
    }

    fn update_price(&self, s: &mut State) {
        if !s.me().bank.has_fixed_resource_price(&self.kind) {
            *s.me_mut().bank.resource_price.get_mut(&self.kind).unwrap() = DEFAULT_RESOURCE_PRICE * s.enemy().resources[&self.kind];
        }
    }
}

impl Effect for ProduceResource {
    fn apply(&self, s: &mut State) {
        *s.me_mut().resources.get_mut(&self.kind).unwrap() += self.count;
        self.update_price(s);
    }

    fn rollback(&self, s: &mut State) {
        let current = s.me().resources[&self.kind];
        *s.me_mut().resources.get_mut(&self.kind).unwrap() = min(current - self.count, 0);
        self.update_price(s);
    }
}

pub struct Science {
    pub symbol: ScientificSymbol,
}

impl Effect for Science {
    fn apply(&self, s: &mut State) {
        let pos = s.me()
            .scientific_symbols
            .iter()
            .position(|(s, _)| *s == self.symbol);

        if let Some(v) = pos {
            s.me_mut().scientific_symbols[v].1 += 1;

            if s.me().scientific_symbols[v].1 == SAME_SCIENTIFIC_SYMBOLS_FOR_TOKEN {
                s.post_effects.push(Box::new(PostPickBoardToken{
                    actor: s.players.me.clone(),
                }));
            }
        } else {
            s.me_mut().scientific_symbols.push((self.symbol, 1));
        }

        if s.me().scientific_symbols.len() == DIFFERENT_SCIENTIFIC_SYMBOLS_FOR_SUPREMACY as usize {
            s.over(Victory::ScienceSupremacy, s.players.me.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_change_coins() {
        let mut s = State {
            phase: Phase::WondersSelection,
            turn: "user1".to_string(),
            cities: HashMap::from([
                ("user1".to_string(), City::default()),
                ("user2".to_string(), City::default()),
            ]),
            post_effects: Vec::new(),
            interactive_units: Units::default(),
        };

        let effects = vec![
            Reward { coins: 3 }
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me_mut().coins, 3);
        assert_eq!(s.enemy_mut().coins, 0);
    }

    #[test]
    fn test_change_coins_negative() {
        let mut s = State {
            phase: Phase::WondersSelection,
            turn: "user1".to_string(),
            cities: HashMap::from([
                ("user1".to_string(), City::default()),
                ("user2".to_string(), City::default()),
            ]),
            post_effects: Vec::new(),
            interactive_units: Units::default(),
        };

        let effects = vec![
            Reward { coins: -3 }
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me_mut().coins, 0);
        assert_eq!(s.enemy_mut().coins, 0);
    }
}