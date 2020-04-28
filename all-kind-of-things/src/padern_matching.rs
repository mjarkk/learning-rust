pub fn run() {
  pattern_matching();
}

fn how_many(x: i32) -> &'static str {
  match x {
    0 => "no",
    1 => "one",
    2 => "two",
    3..=7 => "a few",
    _ if (x < 10) => "a lot",
    _ => "a shitload",
  }
}

fn pattern_matching() {
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x))
  }
}
