use std::collections::{HashMap, HashSet};
use std::ops::IndexMut;
use crate::{*};

#[derive(Default, Debug)]
pub struct Deck {
    pub graph: HashMap<building::Id, Child>,
    pub buildings: Vec<building::Id>,
    pub face_down: HashSet<building::Id>,
}

impl Deck {
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

    pub fn new(age: Age, buildings: Vec<building::Id>) -> Self {
        let mut deck = Deck::default();
        let layout = Self::get_layout(age);
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
                }

                line_pos += 1;
            }

            scheme.push(line);
        }

        let mut it = scheme.iter().peekable();

        for line in it {
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

                    deck.graph.insert(*id, nodes);
                }
            }
        }

        // for (pos, line) in scheme.iter().enumerate() {
        //     for (pos, slot) in line.iter().enumerate() {
        //         if let Some(id) = slot {
        //             deck.graph.insert(*id, Default::default());
        //         }
        //
        //         if
        //
        //         // if
        //     }
        // }

        // let mut deck = Deck::default();
        //
        // let mut prev_line: Line = Default::default();
        // let mut current_line: Line = Default::default();
        // let mut in_row_pos = 0usize;
        // let row_n = 1usize;
        // let mut building_pos = 0usize;
        //
        // for line in layout.lines().skip(1) {
        //     for char in line.chars() {
        //         if char == '[' {
        //             let bid = buildings[building_pos];
        //             current_line.insert(in_row_pos, bid);
        //             deck.graph.insert(bid, Default::default());
        //
        //             if let Some(top_right) = prev_line.get(&(in_row_pos + 1)) {
        //                 deck.graph.get_mut(top_right)[]
        //             }
        //         }
        //
        //         in_row_pos += 1;
        //     }
        //
        //     in_row_pos = 0;
        //     prev_line = current_line;
        //     current_line = Default::default();
            // for char in line.chars() {
            //     if char == '[' {
            //         println!("char");
            //     }
            //
            //     if char.is_whitespace() {
            //         println!("_");
            //     }
            // }
        // }

        // for char in layout.chars() {
        //     match char {
        //         '\n' => {
        //             current_line
        //                 .iter()
        //                 .for_each(|(pos, id)| {
        //                     deck.graph.insert(*id, [None; 2]);
        //
        //                     // let top_left_building = prev_line.get(&(pos + 1));
        //
        //                     if let Some(top_left_building ) = prev_line.get(&(pos + 1)) {
        //                         if top_left_building > 0 {
        //                             deck.graph.get_mut(top_left_building).unwrap()[0] = Some(*id);
        //                         }
        //                     }
        //
        //                     if prev_line[pos - 1] > 0 {
        //                         deck.graph.get_mut(prev_line[pos + 1]).unwrap()[0] = Some(*id);
        //                     }
        //                 })
        //         }
        //         '[' => {
        //             if row_n % 2 == 0 {
        //                 deck.face_down.insert(deck.buildings[building_pos]);
        //             }
        //
        //             current_line.get_mut(&in_row_pos).unwrap() = deck.buildings[building_pos];
        //             building_pos += 1;
        //             in_row_pos += 1;
        //         }
        //         _ => {
        //             in_row_pos += 1;
        //         }
        //     }
        // }
        //
        Self {
            graph: Default::default(),
            buildings,
            face_down: Default::default(),
        }
    }

    pub fn get_layout(age: Age) -> &'static str {
        Self::LAYOUTS[age as usize - 1].trim()
    }

    pub fn get_returned_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }

    pub fn get_top_line_buildings(&self) -> Vec<building::Id> {
        unimplemented!()
    }

    pub fn pull_building(id: building::Id) {}
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