use std::collections::HashMap;

fn main() {
    let store: HashMap<usize, u8> = HashMap::from([
        (1, 20),
        (2, 10),
        (3, 50),
        (4, 40),
        (5, 30),
    ]);

    let mut out  = store.iter()
        .collect::<Vec<_>>();

    out.sort_by(|&a, &b| {b.1.cmp(a.1)});

    let sorted = out.iter()
        .map(|item| item.0)
        .collect::<Vec<_>>();

    println!("{:?}", out);
    println!("{:?}", sorted);
}