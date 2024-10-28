
mod prelude;
mod effect;
mod state;
mod player;
mod building;
mod wonder;
mod token;
mod economy;
mod military;
mod deck;
mod action;
mod rule;

use prelude::*;

pub trait BaseUnit {
    fn effects(&self) -> &Vec<Effect>;

    fn construct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.apply(s)
        }
    }

    fn destruct(&self, s: &mut State) {
        for effect in self.effects() {
            effect.rollback(s)
        }
    }

    fn get_points(&self, s: &State) -> Points {
        self.effects().iter().fold(0, |acc, item| acc + item.get_points(s))
    }
}

#[derive(Debug)]
pub enum Error {
    ActionNotAllowed,
    NotEnoughCoins,
}

#[derive(Default)]
pub struct Options {
    pub with_promo_wonders: bool,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{
        action::{
            Action::*,
            Setup,
        },
        building::Id::*,
        prelude::*,
        token::Id::*,
        wonder::Id::*,
        state::Score,
    };

    #[test]
    fn game_11() {
        let actions = vec![
            Prepare(Setup{
                p1: 1,
                p2: 2,
                wonders: vec![
                    TheHangingGardens,
                    TheTempleOfArtemis,
                    TheColossus,
                    Messe,
                    ThePyramids,
                    StatueOfLiberty,
                    TheMausoleum,
                    TheSphinx,
                ],
                board_tokens: vec![
                    Economy,
                    Agriculture,
                    Philosophy,
                    Theology,
                    Law,
                ],
                random_tokens: vec![
                    Urbanism,
                    Strategy,
                    Masonry,
                ],
                buildings: HashMap::from([
                    (
                        Age::I,
                        vec![
                            Palisade,
                            Theater,
                            Tavern,
                            Stable,
                            Altar,
                            Workshop,
                            ClayReserve,
                            GlassWorks,
                            LoggingCamp,
                            LumberYard,
                            Baths,
                            Quarry,
                            ClayPit,
                            ClayPool,
                            Scriptorium,
                            Garrison,
                            StonePit,
                            WoodReserve,
                            Pharmacist,
                            StoneReserve,
                        ],
                    ),
                    (
                        Age::II,
                        vec![
                            Dispensary,
                            CustomHouse,
                            CourtHouse,
                            Caravansery,
                            GlassBlower,
                            BrickYard,
                            School,
                            Laboratory,
                            Aqueduct,
                            ArcheryRange,
                            ParadeGround,
                            Brewery,
                            Statue,
                            HorseBreeders,
                            ShelfQuarry,
                            Library,
                            Walls,
                            SawMill,
                            Barracks,
                            DryingRoom,
                        ],
                    ),
                    (
                        Age::III,
                        vec![
                            Port,
                            Academy,
                            Obelisk,
                            Observatory,
                            Fortifications,
                            Palace,
                            Senate,
                            Armory,
                            MagistratesGuild,
                            MerchantsGuild,
                            SiegeWorkshop,
                            ChamberOfCommerce,
                            Arsenal,
                            Pretorium,
                            Arena,
                            Lighthouse,
                            Gardens,
                            Pantheon,
                            MoneyLendersGuild,
                            TownHall,
                        ],
                    )
                ]),
            }),
            PickWonder(TheTempleOfArtemis),//1
            PickWonder(TheHangingGardens),//2
            PickWonder(TheColossus),//3
            PickWonder(Messe),//4
            PickWonder(TheSphinx),//5
            PickWonder(StatueOfLiberty),//6
            PickWonder(TheMausoleum),//7
            PickWonder(ThePyramids),//8
            ConstructBuilding(WoodReserve),//9
            ConstructBuilding(StoneReserve),//10
            ConstructBuilding(Scriptorium),//11
            ConstructBuilding(StonePit),//12
            ConstructBuilding(Quarry),//13
            DiscardBuilding(Garrison),//14
            ConstructBuilding(Pharmacist),//15
            ConstructBuilding(ClayPool),//16
            ConstructBuilding(LumberYard),//17
            ConstructBuilding(Baths),//18
            DiscardBuilding(ClayPit),//19
            ConstructBuilding(LoggingCamp),//20
            ConstructBuilding(GlassWorks),//21
            ConstructBuilding(Altar),//22
            ConstructBuilding(Workshop),//23
            DiscardBuilding(ClayReserve),//24
            ConstructBuilding(Tavern),//25
            ConstructBuilding(Stable),//26
            ConstructBuilding(Theater),//27
            ConstructBuilding(Palisade),//28
            SelectWhoBeginsTheNextAge(1),//29
            ConstructBuilding(DryingRoom),//30
            ConstructBuilding(SawMill),//31
            ConstructBuilding(ShelfQuarry),//32
            DiscardBuilding(ParadeGround),//33
            ConstructBuilding(BrickYard),//34
            ConstructBuilding(Barracks),//35
            ConstructBuilding(Library),//36
            PickBoardToken(Theology),//37
            ConstructBuilding(Walls),//38
            ConstructBuilding(Brewery),//39
            DiscardBuilding(HorseBreeders),//40
            ConstructWonder(Messe, Statue),//41
            PickTopLineBuilding(Dispensary),//42
            PickBoardToken(Economy),//43
            ConstructBuilding(Laboratory),//44
            PickBoardToken(Agriculture),//45
            ConstructBuilding(ArcheryRange),//46
            ConstructBuilding(Aqueduct),//47
            ConstructBuilding(GlassBlower),//48
            ConstructBuilding(School),//49
            DiscardBuilding(CourtHouse),//50
            ConstructBuilding(Caravansery),//51
            ConstructBuilding(CustomHouse),//52
            SelectWhoBeginsTheNextAge(1),//53
            ConstructWonder(TheMausoleum, MoneyLendersGuild),//54
            PickDiscardedBuilding(ParadeGround),//55
            ConstructBuilding(Lighthouse),//56
            ConstructBuilding(ChamberOfCommerce),//57
            ConstructBuilding(TownHall),//58
            ConstructWonder(ThePyramids, Gardens),//59
            ConstructBuilding(Arsenal),//60
            DiscardBuilding(Pantheon),//61
            DiscardBuilding(Pretorium),//62
            ConstructBuilding(MerchantsGuild),//63
            ConstructWonder(StatueOfLiberty, Senate),//64
            PickReturnedBuildings(Study, Circus),//65
            ConstructWonder(TheTempleOfArtemis, Palace),//66
            ConstructBuilding(Obelisk),//67
            ConstructBuilding(Arena),//68
            ConstructBuilding(SiegeWorkshop),//69
            ConstructBuilding(MagistratesGuild),//70
            ConstructBuilding(Armory),//71
            ConstructBuilding(Observatory),//72
            ConstructBuilding(Fortifications),//73
            ConstructBuilding(Port),//74
            ConstructBuilding(Academy),//75
            PickBoardToken(Philosophy),//76
        ];
        let s = State::from(actions).expect("its ok");

        // coins 33
        let expected1 = Score{
            civilian: 20,
            science: 13,
            commercial: 6,
            guilds: 0,
            wonders: 9,
            tokens: 11,
            coins: 11,
            military: 0,
            total: 70,
        };

        // coins 19
        let expected2 = Score{
            civilian: 6,
            science: 2,
            commercial: 9,
            guilds: 10,
            wonders: 9,
            tokens: 0,
            coins: 6,
            military: 0,
            total: 42,
        };

        let p1 = s.enemy();
        let p2 = s.me();

        assert_eq!(expected1, p1.score);
        assert_eq!(33, p1.coins);

        assert_eq!(expected2, p2.score);
        assert_eq!(19, p2.coins);
    }
}