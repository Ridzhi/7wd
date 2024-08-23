use std::cmp::max;
use crate::{
    *,
    state::{State, City, Units},
    building
};

pub enum Effect {
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
    DestructBuilding{
        kind: building::Kind
    },
}

impl Effect {
    pub fn mutate(&self, s: & mut State) {
        match *self {
            Effect::Chain { building } => {
                s.me().chains.push(building);
            },

            Effect::Coins { count } => {
                let next = s.me().coins as i8 + count;
                // treasure can't be negative
                s.me().coins = max(next, 0) as u8;
            },

            Effect::CoinsFor { count, bonus } => {
                let next = s.me().coins + (s.me().bonus_rate(bonus) * count);
                s.me().coins = next;
            }
            Effect::DestructBuilding { kind } => {
                let buildings = building::filter_by_kind(&s.enemy().buildings, kind);

                if buildings.is_empty() {
                    return;
                }

                s.interactive_effects.push(InteractiveEffect::DestructBuilding {
                    player: &s.turn,
                    buildings,
                })

                // push dialog
            }
        }
    }

    pub fn rollback(&self, _s: &mut State) {
        ()
    }

    pub fn points(&self, _s: &State) -> u8 {
        0
    }
}

pub enum InteractiveEffect {
    DestructBuilding {
        player: Nickname,
        buildings: Vec<building::Id>,
    }
}

impl InteractiveEffect {
    pub fn mutate(&self, s: &mut State) {
        match self {
            InteractiveEffect::DestructBuilding{player, buildings} => {
                s.phase = Phase::DestructBuildingSelection;
                s.set_turn(player);
                s.interactive_units.buildings = Some(buildings);
            }
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
            interactive_effects: Vec::new(),
            interactive_units: Units::default(),
        };

        let effects = vec![
            Effect::Coins { count: 3}
        ];

        effects.iter().for_each(|eff| eff.mutate(&mut s));

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
            interactive_effects: Vec::new(),
            interactive_units: Units::default(),
        };

        let effects = vec![
            Effect::Coins { count: -3}
        ];

        effects.iter().for_each(|eff| eff.mutate(&mut s));

        assert_eq!(s.me().coins, 0);
        assert_eq!(s.enemy().coins, 0);
    }
}