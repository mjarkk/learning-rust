use std::mem;

struct Point {
  x: f64,
  y: f64,
}

fn origin() -> Point {
  Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
  let p1 = origin();
  let p2 = Box::new(origin());
  let p1_size = mem::size_of_val(&p1);
  let p2_size = mem::size_of_val(&p2);
  println!("size of p1: {}, size of p2: {}", p1_size, p2_size);
}
