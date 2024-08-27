use std::cmp::{max, min};
use crate::{
    building,
    state::{City, State, Units},
    *,
};
use crate::economy::{Coins, Discount, PayScope};

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

pub struct Chain {
    pub building: building::Id,
}

impl Effect for Chain {
    fn apply(&self, s: &mut State) {
        s.me().chains.push(self.building);
    }
}

pub struct Reward {
    coins: Coins,
}

impl Effect for Reward {
    fn apply(&self, s: &mut State) {
        s.me().coins += self.coins;
    }
}

pub struct RewardFor {
    pub coins: Coins,
    pub bonus: Bonus,
}

impl Effect for RewardFor {
    fn apply(&self, s: &mut State) {
        s.me().coins += s.me().bonus_rate(self.bonus) * self.coins;
    }
}

pub struct DestructBuilding {
    pub kind: building::Kind,
}

impl Effect for DestructBuilding {
    fn apply(&self, s: &mut State) {
        let buildings = building::filter_by_kind(&s.enemy().buildings, self.kind);

        if buildings.is_empty() {
            return;
        }

        s.post_effects.push(Box::new(
            PostDestructBuilding {
                player: s.turn.clone(),
                buildings,
            }
        ));
    }
}

pub struct DiscardRewardAdjuster;

impl Effect for DiscardRewardAdjuster {
    fn apply(&self, s: &mut State) {
        s.me().bank.discard_reward += 1;
    }
}

pub struct Discounter {
    scope: PayScope,
    resources: Vec<Resource>,
    count: u8,
}

impl Effect for Discounter {
    fn apply(&self, s: &mut State) {
        s.me().bank.discounts.push(Discount {
            scope: self.scope,
            resources: self.resources.clone(),
            count: self.count,
        })
    }
}

pub struct Fine {
    pub coins: Coins,
}

impl Effect for Fine {
    fn apply(&self, s: &mut State) {
        s.enemy().coins -= min(self.coins, s.enemy().coins);
    }
}

pub struct FixedPrice {
    pub resources: Vec<Resource>,
}

impl Effect for FixedPrice {
    fn apply(&self, s: &mut State) {
        self.resources.iter()
            .for_each(|resource| {
                *s.me().bank.resource_price.get_mut(resource).unwrap() = FIXED_RESOURCE_PRICE;
            })
    }
}

pub struct Guild {
    pub bonus: Bonus,
    pub points: u8,
    pub coins: Coins,
}

impl Guild {
    fn rate(&self, s: &mut State) -> u8 {
        max(s.me().bonus_rate(self.bonus), s.enemy().bonus_rate(self.bonus))
    }
}

impl Effect for Guild {
    fn apply(&self, s: &mut State) {
        s.me().coins += self.rate(s) * self.coins;
    }

    fn get_points(&self, s: &mut State) -> u8 {
        self.rate(s) * self.points
    }
}

pub struct Mathematics;

impl Effect for Mathematics {
    fn get_points(&self, s: &mut State) -> u8 {
        s.me().tokens.len() as u8 * 3
    }
}

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

        let (fine, supremacy) = s.move_conflict_pawn(self.power);

        // let mut my_track = &mut s.me().track;
        // let (fine, supremacy) = my_track.move_conflict_pawn(s, self.power);

        if fine > 0 {
            s.enemy().coins -= min(fine, s.enemy().coins);
        }
    }
}

pub struct PostDestructBuilding {
    player: Nickname,
    buildings: Vec<building::Id>,
}

impl PostEffect for PostDestructBuilding {
    fn apply(self, s: &mut State) {
        s.phase = Phase::DestructBuildingSelection;
        s.set_turn(self.player);
        s.interactive_units.buildings = self.buildings;
    }
}

pub enum Effectv2 {
    Chain {
        building: building::Id,
    },
    Coins {
        count: i8
    },
    CoinsFor {
        count: u8,
        bonus: Bonus,
    },
    DestructBuilding {
        kind: building::Kind
    },
    DiscardRewardAdjuster,
    Discounter {
        scope: PayScope,
        resources: Vec<Resource>,
        count: u8,
    },
    // Guild { bonus: Bonus, points: u8, coins: u8 },
    // Mathematics {},
    // Military { power: u8, strategy_disabled: bool },
    // PickBoardToken {},
    // PickDiscardedCard {},
    // PickRandomToken {},
    // PickReturnedCards {},
    // PickTopLineCard {},
    // PlayAgain {},
    // Points { count: u8 },
    // Resource { resource: RId, count: u8 },
    // Science { symbol: ScientificSymbol },
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

        assert_eq!(s.me().coins, 3);
        assert_eq!(s.enemy().coins, 0);
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

        assert_eq!(s.me().coins, 0);
        assert_eq!(s.enemy().coins, 0);
    }
}