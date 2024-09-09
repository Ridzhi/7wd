pub type Nickname = u8;

pub enum Finisher {
    Winner(Nickname),
    Loser(Nickname),
}