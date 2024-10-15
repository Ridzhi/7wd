use std::collections::{HashMap, HashSet};
use crate::{*};

#[derive(Default, Debug)]
pub struct Deck {
    pub graph: HashMap<building::Id, [Option<building::Id>; 2]>,
    pub buildings: HashMap<Age, Vec<building::Id>>,
    pub face_down: HashSet<building::Id>,
}

impl Deck {
    // fake line(will skipped) to keep whitespaces after String.lines()
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

    pub fn new(age: Age) -> Self {
        // unimplemented!();
        let mut deck = Self::default();
        let layout = Self::get_layout(age);

        let mut prev_line: Line = Default::default();
        let mut current_line: Line = Default::default();
        let mut in_row_pos = 0usize;
        let row_n = 1usize;
        let mut building_pos = 0usize;

        for line in layout.lines().skip(1) {
            // for char in line.chars() {
            //
            //     if char == '[' {
            //         println!("char");
            //     }
            //
            //     if char.is_whitespace() {
            //         println!("_");
            //     }
            // }
            println!("{}", line);
        }

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
        deck
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

type Line = HashMap<usize, building::Id>;