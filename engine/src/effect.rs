use std::cmp::max;
use crate::{
    Bonus,
    state::{State, City},
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
        group: building::Kind
    },
}

impl Effect {
    pub fn apply(&self, s: &mut State) {
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

            }
            Effect::DestructBuilding { .. } => {}
        }
    }

    pub fn discard(&self, _s: &mut State) {
        ()
    }

    pub fn points(&self, _s: &State) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_change_coins() {
        let mut s = State {
            turn: "user1".to_string(),
            cities: HashMap::from([
                ("user1".to_string(), City::default()),
                ("user2".to_string(), City::default()),
            ]),
        };

        let effects = vec![
            Effect::Coins { count: 3}
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me().coins, 3);
        assert_eq!(s.enemy().coins, 0);
    }

    #[test]
    fn test_change_coins_negative() {
        let mut s = State {
            turn: "user1".to_string(),
            cities: HashMap::from([
                ("user1".to_string(), City::default()),
                ("user2".to_string(), City::default()),
            ]),
        };

        let effects = vec![
            Effect::Coins { count: -3}
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me().coins, 0);
        assert_eq!(s.enemy().coins, 0);
    }
}