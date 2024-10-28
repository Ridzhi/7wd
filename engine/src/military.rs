use crate::{*};

pub type Pos = u8;

#[derive(Default, Debug)]
pub struct Track {
    pub pos: Pos,
    pub max_zone: usize,
}

impl Track {
    pub const CAPITAL_POS: u8 = 9;
    // Токены штрафа это переменная которую можно выразить через эффекты,
    // в агоре например, нет штрафов, там токены влияния
    // Соответственно констуктор трека должен принимать токены зон, которые
    // состоят из эффектов
    // Либо есть трейт трека, с дефолтной реализацией, и в экстеншине идет
    // нужный оверрайд
    pub const ZONES: [Zone; 4] = [
        // start pos, points, coins
        Zone(0, 0, 0),
        Zone(1, 2, 0),
        Zone(3, 5, 2),
        Zone(6, 10, 5),
    ];

    pub fn move_conflict_pawn(&mut self, s: &mut State, power: u8) -> (Coins, bool) {
        let mut fine: Coins = 0;
        let mut supremacy = false;
        let mut enemy_pos = &mut s.enemy_mut().track.pos;

        if *enemy_pos >= power {
            *enemy_pos -= power;

            return (fine, supremacy);
        }

        self.pos += power - *enemy_pos;
        *enemy_pos = 0;

        if self.pos >= Self::CAPITAL_POS {
            self.pos = Self::CAPITAL_POS;
            supremacy = true;
        }

        let zone_index = self.get_zone_index();

        if zone_index > self.max_zone {
            self.max_zone = zone_index;
            fine = Self::ZONES[zone_index].2;
        }

        (fine, supremacy)
    }

    pub fn get_points(&self) -> u8 {
        Self::ZONES[self.get_zone_index()].1
    }

    pub fn get_zone_index(&self) -> usize {
        let ind = [3,2,1].iter().find(|&&ind| {
            self.pos >= Self::ZONES[ind].0
        });

        if let Some(v) = ind {
            return *v
        };

        0
    }
}

pub struct Zone(pub Pos, pub Points, pub Coins);
