use std::collections::{HashMap, HashSet};
use crate::{*};

// fake line(will skipped) to keep leading whitespaces after String.lines()
pub const LAYOUTS: [&'static str; 3] = [
    r#"
------------
    [][]
   [][][]
  [][][][]
 [][][][][]
[][][][][][]
        "#,
    r#"
------------
[][][][][][]
 [][][][][]
  [][][][]
   [][][]
    [][]
        "#,
    r#"
--------
  [][]
 [][][]
[][][][]
 []  []
[][][][]
 [][][]
  [][]
        "#,
];

pub fn get_layout(age: Age) -> &'static str {
    LAYOUTS[age as usize - 1].trim()
}

#[derive(Default, Debug)]
pub struct Deck {
    pub buildings: Vec<building::Id>,
    pub graph: HashMap<building::Id, Child>,
    pub face_down: HashSet<building::Id>,
}

impl Deck {
    pub fn new(layout: &str, buildings: Vec<building::Id>) -> Self {
        let scheme = Self::build_scheme(layout, &buildings);
        let graph = Self::build_graph(&scheme);

        let face_down = scheme.iter()
            .skip(1)
            .step_by(2)
            .flatten()
            .cloned()
            .flatten()
            .collect();

        Self {
            buildings,
            graph,
            face_down,
        }
    }

    pub fn get_returned_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }

    pub fn get_top_line_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }

    pub fn pull_building(id: building::Id) {}

    fn build_scheme(layout: &str, buildings: &Vec<building::Id>) -> Vec<Line> {
        let mut scheme: Vec<Line> = Vec::with_capacity(layout.lines().count());
        let mut building_pos = 0usize;

        for layout_line in layout.lines().skip(1) {
            let mut line = Line::default();
            let mut line_pos = 0usize;

            for char in layout_line.chars() {
                if char == '[' {
                    // @TODO research how to insert in one expr
                    line[line_pos] = Some(buildings[building_pos]);
                    line[line_pos + 1] = Some(buildings[building_pos]);
                    building_pos += 1;
                }

                line_pos += 1;
            }

            scheme.push(line);
        }

        scheme
    }

    fn build_graph(scheme: &Vec<Line>) -> HashMap<building::Id, Child> {
        let mut graph: HashMap<building::Id, Child> = Default::default();
        let mut it = scheme.iter().peekable();

        while let Some(line) = it.next() {
            for (pos, slot) in line.iter().enumerate()  {
                if let Some(id) = slot {
                    let mut nodes: Child = Default::default();

                    if let Some(next) = it.peek() {
                        if let Some(left) = next[pos-1] {
                            nodes[0] = Some(left)
                        }

                        if let Some(right) = next[pos + 1] {
                            nodes[1] = Some(right)
                        }
                    }

                    graph.insert(*id, nodes);
                }
            }
        }

        graph
    }
}

pub type Layout = Vec<Slot>;

#[derive(Default, Debug)]
pub enum Slot {
    #[default]
    Empty,
    FaceDown,
    FaceDownGuild,
    FaceUp(building::Id),
}

// track in which positions placed item
// each building keep 2 slots
// suggest 10 buildings is enough(6 max currently)
type Line = [Option<building::Id>; 20];
type Child = [Option<building::Id>; 2];

#[cfg(test)]
mod tests {
    use super::*;
    use building::Id::*;

    #[test]
    fn check_new() {
        let d = Deck::new(get_layout(Age::I), vec![
            LumberYard,//100
            LoggingCamp,//101
            ClayPool,//102
            ClayPit,//103
            Quarry,//104
            StonePit,//105
            GlassWorks,//106
            Press,//107
            GuardTower,//108
            Workshop,//109
            Apothecary,//110
            StoneReserve,//111
            ClayReserve,//112
            WoodReserve,//113
            Stable,//114
            Garrison,//115
            Palisade,//116
            Scriptorium,//117
            Pharmacist,//118
            Theater,//119
        ]);

        assert_eq!(
            d.face_down,
            HashSet::from([
                ClayPool,
                ClayPit,
                Quarry,
                Workshop,
                Apothecary,
                StoneReserve,
                ClayReserve,
                WoodReserve,
            ]),
        );

        assert_eq!(
            d.graph,
            HashMap::from([
                (LumberYard, [Some(ClayPool), Some(ClayPit)]),
                (LoggingCamp, [Some(ClayPit), Some(Quarry)]),
                (ClayPool, [Some(StonePit), Some(GlassWorks)]),
                (ClayPit, [Some(GlassWorks), Some(Press)]),
                (Quarry, [Some(Press), Some(GuardTower)]),
                (StonePit, [Some(Workshop), Some(Apothecary)]),
                (GlassWorks, [Some(Apothecary), Some(StoneReserve)]),
                (Press, [Some(StoneReserve), Some(ClayReserve)]),
                (GuardTower, [Some(ClayReserve), Some(WoodReserve)]),
                (Workshop, [Some(Stable), Some(Garrison)]),
                (Apothecary, [Some(Garrison), Some(Palisade)]),
                (StoneReserve, [Some(Palisade), Some(Scriptorium)]),
                (ClayReserve, [Some(Scriptorium), Some(Pharmacist)]),
                (WoodReserve, [Some(Pharmacist), Some(Theater)]),
                (Stable, [None, None]),
                (Garrison, [None, None]),
                (Palisade, [None, None]),
                (Scriptorium, [None, None]),
                (Pharmacist, [None, None]),
                (Theater, [None, None]),
            ]),
        );
    }
}