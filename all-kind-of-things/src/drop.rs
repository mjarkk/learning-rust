pub fn run() {
  {
    let scary = Beest::new("Scary beest whoooooo");
    println!("Beest name: {}", scary.name);
    // scary gets dropped here
  }

  let scary = Beest::new("Scary beest whoooooo");
  println!("Beest name: {}", scary.name);
  // manually drop the beest
  drop(scary);
}

struct Beest {
  name: String,
}

impl Beest {
  fn new<S: Into<String>>(name: S) -> Self {
    Beest { name: name.into() }
  }
}

impl Drop for Beest {
  fn drop(&mut self) {
    println!("\"{}\" is removed", self.name);
  }
}
