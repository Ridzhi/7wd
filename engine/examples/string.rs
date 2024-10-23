#[derive(Default)]
struct State {
    pub points: usize,
    pub p1: City,
    pub p2: City,
}

#[derive(Default)]
struct City {
    pub coins: usize,
}



fn main() {
    let s = State::default();
}