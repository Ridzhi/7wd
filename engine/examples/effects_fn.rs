use std::collections::HashMap;
use std::sync::LazyLock;

type Mutator = Box<dyn Fn(&mut State) + Send + Sync>;

#[derive(Default, Debug)]
struct State {
    pub x: u8,
    pub y: u8,
}

fn add_x(count: u8) -> Mutator {
    fn inc(x: u8) -> u8 {
        x + 1
    }

    Box::new(move |s| s.x += inc(count))
}

fn add_y(count: u8) -> Mutator {
    Box::new(move |s| s.y += count)
}

struct Unit {
    pub effects: Vec<Mutator>
}

impl Unit {
    pub fn apply(&self, s: &mut State) {
        self.effects.iter().map(|e| e(s)).collect()
    }
}

static REGISTRY: LazyLock<HashMap<String, Unit>> = LazyLock::new(|| {
    HashMap::from([
        ("forum".to_string(), Unit {
            effects: vec![
                add_x(1),
                add_y(2),
            ],
        })
    ])
});

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