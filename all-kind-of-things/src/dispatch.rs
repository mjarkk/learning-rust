struct Human {
  name: String,
}

struct Beest {
  name: String,
}

enum Creature {
  Human(Human),
  Beest(Beest),
}

pub fn run() {
  let a = 123;
  let b = "hello".to_string();

  print1(&a);
  print1(&b);

  print2(&a);
  print2(&b);

  let items = vec![
    Creature::Beest(Beest {
      name: String::from("Beest"),
    }),
    Creature::Human(Human {
      name: String::from("Human"),
    }),
  ];
  for item in items {
    let name = match item {
      Creature::Beest(a) => a.name,
      Creature::Human(a) => a.name,
    };
    println!("name {}", name);
  }
}

fn print1<T: Printable>(to_print: &T) {
  println!("{}", to_print.format());
}

fn print2(to_print: &dyn Printable) {
  println!("{}", to_print.format());
}

trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
    format! {"i32: {}", self}
  }
}

impl Printable for String {
  fn format(&self) -> String {
    format! {"String: {}", self}
  }
}
