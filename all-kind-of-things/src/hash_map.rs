use std::collections::HashMap;

pub fn run() {
  let mut shapes = HashMap::new();
  shapes.insert(String::from("triangle"), 3);
  shapes.insert(String::from("square"), 3);
  shapes.insert("square".into(), 4);

  println!("A square has {} sidex", shapes["square"]);

  for (key, val) in &shapes {
    println!("A {} has {} sidex", key, val);
  }

  println!("{}", shapes.entry("square".into()).or_insert(1));
  println!("{}", shapes.entry("circle".into()).or_insert(1));

  {
    let actual = shapes.entry("circle".into()).or_default();
    *actual = 0;
  }

  println!("{:?}", shapes);
}
