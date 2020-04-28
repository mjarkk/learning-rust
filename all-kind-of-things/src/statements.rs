pub fn somethings() {
  let temp = {
    let inside = 20;
    let outside = 10;
    outside + inside
  };
  let _is_warn = if temp > 25 { "Yes" } else { "Nope" };

  let mut x = 1;
  while x < 10 {
    print!("{} ", x);
    x += 1;
  }
  println!("");

  loop {
    break;
  }

  for x in 0..10 {
    if x == 3 {
      continue;
    }

    if x == 8 {
      break;
    }

    print!("{} ", x);
  }
  println!("");

  let arr: [u8; 4] = [23, 56, 12, 25];
  for (i, item) in arr.iter().enumerate() {
    println!("{}: {}", i, item)
  }
}
