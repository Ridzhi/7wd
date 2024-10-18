use std::collections::{HashMap, HashSet};
use crate::{*};

#[derive(Default, Debug)]
pub struct Deck {
    pub buildings: Vec<building::Id>,
    pub graph: HashMap<building::Id, Child>,
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
        // let mut deck = Deck::default();
        let scheme = Self::build_scheme(age, &buildings);
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

    fn build_scheme(age: Age, buildings: &Vec<building::Id>) -> Vec<Line> {
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