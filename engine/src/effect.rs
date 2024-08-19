use std::cmp::max;
use crate::{
    state::{State, City},
};

enum Effect {
    ChangeCoins{coins: i8},
}

impl Effect {
    pub fn apply(&self, s: &mut State) {
        match *self {
            Effect::ChangeCoins { coins } => {
                let next = s.me().treas.coins as i8 + coins;
                // treasure can't be negative
                s.me().treas.coins = max(next, 0) as u8;
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
            turn: "user1".to_string(),
            cities: HashMap::from([
                ("user1".to_string(), City::default()),
                ("user2".to_string(), City::default()),
            ]),
        };

        let effects = vec![
            Effect::ChangeCoins {coins: 3}
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me().treas.coins, 3);
        assert_eq!(s.enemy().treas.coins, 0);
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
            Effect::ChangeCoins {coins: -3}
        ];

        effects.iter().for_each(|eff| eff.apply(&mut s));

        assert_eq!(s.me().treas.coins, 0);
        assert_eq!(s.enemy().treas.coins, 0);
    }
}