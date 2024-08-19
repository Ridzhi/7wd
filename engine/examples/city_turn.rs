use std::collections::HashMap;

fn main() {
    let mut state = HashMap::new();
    state.insert("user1", "city1");
    state.insert("user2", "city2");


    let enemy = state.keys().find(|&&k| !k.eq("user1")).unwrap();
    println!("{}", enemy);

    println!("turn check");
}