use std::collections::HashSet;

pub fn run() {
  let mut greeks = HashSet::new();
  greeks.insert("gamma");
  greeks.insert("delta");

  println!("{:?}", greeks);

  for item in &greeks {
    println!("{}", item);
  }

  let to_add = ["vega", "delta"];
  for item_to_add in to_add.iter() {
    let inserted = greeks.insert(item_to_add);
    if inserted {
      println!("Added {}", item_to_add);
    } else {
      println!("{} already inside hash set", item_to_add);
    };
  }

  greeks.remove("gamma");
  println!("{:?}", greeks);

  let one_to_ten: HashSet<_> = (0..10).collect();
  println!("{:?}", one_to_ten);
}
