fn main() {
    let layout = r#"
    [][]
   [][][]
  [][][][]
 [][][][][]
[][][][][][]
"#.trim();

    for char in layout.chars() {
        if char == '\n' {
            println!("newline");
        }
    }

    // println!("test string");
}