pub fn run() {
  let s1: &'static str = "Yoyo";
  for c in s1.chars().rev() {
    print!("{}", c);
  }
  println!("");
  let mut s2 = String::new();
  let mut letter = b'a';
  while letter <= b'z' {
    s2.push(letter as char);
    if letter != b'z' {
      s2.push_str(",");
    }
    letter += 1;
  }
  println!("{}", s2);

  let mut s3 = String::from("Hello world");
  s3.remove(0);
  s3.push_str("!!!");
  println!("{}", s3.replace("ello", "Goodby"));

  let name = "Mark";
  let greeting = format!("Hello {}!", name);
  println!("{}", greeting);

  let run = "run";
  let forest = "forest";

  let rfr = format!("{0} {1} {0}", run, forest);
  println!("{}", rfr);

  let info = format!(
    "the name's {last}. {first} {last}",
    first = "james",
    last = "bond",
  );
  println!("{}", info);
}
