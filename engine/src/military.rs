use crate::{*};

type Pos = u8;

#[derive(Default)]
pub struct Track {
    pub pos: Pos,
    pub max_zone: usize,
}

impl Track {
    const CAPITAL_POS: u8 = 9;
    // Токены штрафа это переменная которую можно выразить через эффекты,
    // в агоре например, нет штрафов, там токены влияния
    // Соответственно констуктор трека должен принимать токены зон, которые
    // состоят из эффектов
    // Либо есть трейт трека, с дефолтной реализацией, и в экстеншине идет
    // нужный оверрайд
    const ZONES: [Zone; 4] = [
        // start pos, points, coins
        Zone(0, 0, 0),
        Zone(1, 2, 0),
        Zone(3, 5, 2),
        Zone(6, 10, 5),
    ];

    pub fn move_conflict_pawn(&mut self, s: &mut State, power: u8) {
        if s.enemy().track.pos >= power {
            s.enemy().track.pos -= power;
            return
        }

        self.pos += (power - s.enemy().track.pos);
        s.enemy().track.pos = 0;

        if self.pos >= Self::CAPITAL_POS {
            self.pos = Self::CAPITAL_POS;
            // set state supremacy
        }

        let zone_index = self.get_zone_index();

        if zone_index > self.max_zone {
            self.max_zone = zone_index;
            // apple effects fine
        }
    }

    fn get_zone_index(&self) -> usize {
        let ind = [3,2,1].iter().find(|&&ind| {
            self.pos >= Self::ZONES[ind].0
        });

        if let Some(v) = ind {
            return *v
        };

        0
    }
}

struct Zone(Pos, Points, Coins);
