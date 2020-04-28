use std::mem;

fn vars() {
  let a: isize = 4;
  let _size_of_a = mem::size_of_val(&a);
  // println!("{}-bit os", size_of_a * 8);

  let z_char = 'z';
  println!("{}", z_char.to_string());
}
