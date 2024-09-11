use std::collections::{HashMap, HashSet};
use crate::{*};

#[derive(Default, Debug)]
pub struct Deck {
    pub graph: HashMap<building::Id, [building::Id; 2]>,
    pub buildings: HashMap<Age, Vec<building::Id>>,
    pub face_down: HashSet<building::Id>,
}

impl Deck {
    pub const LAYOUTS: [&'static str; 3] = [
        r#"
    [][]
   [][][]
  [][][][]
 [][][][][]
[][][][][][]
        "#,
        r#"
[][][][][][]
 [][][][][]
  [][][][]
   [][][]
    [][]
        "#,
        r#"
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
        let layout = Self::get_layout(age);

        let prev_line: Line = Default::default();
        let current_line: Line = Default::default();
        let in_row_pos = 0;
        let row_n = 1;
        let building_pos = 0;

        // for char in layout.chars() {
        //     match char { _ => {} }
        // }
        
        unimplemented!()
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

    pub fn pull_building(id: building::Id) {

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

type Line = HashMap<building::Id, usize>;