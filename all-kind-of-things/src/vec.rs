pub fn run() {
  let mut v = vec![8, 7, 6, 5];
  v.push(1);
  v.push(2);
  v.push(3);
  println!("v = {:?}", v);

  match v.get(100) {
    Some(v) => println!("found output on position 100: {}", v),
    None => println!("there is no data on position 100"),
  };

  let out = match v.get(100) {
    Some(v) => v,
    None => &0,
  };

  println!("Data on position 100: {}", out);

  for item in &v {
    println!("{}", item);
  }

  while let Some(x) = v.pop() {
    println!("{}", x);
  }
}
