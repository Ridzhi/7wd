type Mutator = fn(s: &mut State);

#[derive(Default, Debug)]
struct State {
    pub x: u8,
    pub y: u8,
}

fn add_x(count: u8) -> Mutator {
    |s| s.x += count
}

fn add_y(count: u8) -> Mutator {
    |s| s.y += count
}

struct Unit {
    pub effects: Vec<Mutator>
}

impl Unit {
    pub fn apply(&self, s: &mut State) {
        self.effects.iter().map(|e| e(s)).collect()
    }
}

fn main() {
    let forum = Unit {
        effects: vec![
            add_x(1),
            add_y(2),
        ],
    };

    let mut s = State::default();

    forum.apply(&mut s);

    println!("{:?}", s);
}