use std::fmt::Debug;

pub fn run() {
  let a = Animal::create("Dog");
  let h = Human::create("Mark");
  let t: Human = AliveThing::create("lolz");

  h.talk();
  a.talk();
  t.talk();

  let items = vec![1, 2, 3];
  println!("sum: {}", items.sum());

  printy_print(a);
  printy_print(h);
  printy_print(items);

  let p1 = Person::new("person 1"); // &'static str
  let p2 = Person::new(String::from("person 2")); // String
  println!("{}", p1.name);
  println!("{}", p2.name);
}

struct Person {
  name: String,
}

impl Person {
  fn new<S: Into<String>>(name: S) -> Person {
    Person { name: name.into() }
  }
}

trait PrintDatContent {
  fn get_dat_printable_content(&self) -> String;
}

// fn printy_print(boii: impl PrintDatContent + Debug) {
// fn printy_print<T: PrintDatContent + Debug>(boii: T) {
fn printy_print<T>(boii: T)
where
  T: PrintDatContent + Debug,
{
  println!("contents: {}", boii.get_dat_printable_content());
  println!("debug dat boii: {:?}", boii);
}

trait Summable<T> {
  fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
  fn sum(&self) -> i32 {
    let mut out = 0;
    for item in self {
      out += item;
    }
    out
  }
}

impl PrintDatContent for Vec<i32> {
  fn get_dat_printable_content(&self) -> String {
    String::from(format!("items of this boii: {:?}", self))
  }
}

trait AliveThing {
  fn create(name: &'static str) -> Self;
  fn name(&self) -> &'static str;
  fn talk(&self) {
    println!("{} Cannot talk", self.name())
  }
}

#[derive(Debug)]
struct Human {
  name: &'static str,
}

impl AliveThing for Human {
  fn create(name: &'static str) -> Human {
    Human { name: name }
  }
  fn name(&self) -> &'static str {
    self.name
  }
}

impl PrintDatContent for Human {
  fn get_dat_printable_content(&self) -> String {
    String::from(format!("Name for dat human boiiii: {}", self.name()))
  }
}

#[derive(Debug)]
struct Animal {
  name: &'static str,
}

impl AliveThing for Animal {
  fn create(name: &'static str) -> Animal {
    Animal { name: name }
  }
  fn name(&self) -> &'static str {
    self.name
  }
  fn talk(&self) {
    println!("Hello says {}", self.name())
  }
}

impl PrintDatContent for Animal {
  fn get_dat_printable_content(&self) -> String {
    String::from(format!("Name for dat animal: {}", self.name()))
  }
}
