struct Point(f64, f64); // x and y

struct Line {
  start: Point,
  end: Point,
}

impl Line {
  fn len(&self) -> f64 {
    let dx = self.start.0 - self.end.0;
    let dy = self.start.1 - self.end.0;
    (dx * dx + dy * dy).sqrt()
  }
}

pub fn run() {
  numbers();

  let line = Line {
    start: Point(1.0, 1.0),
    end: Point(5.0, 5.0),
  };
  println!("Line lenght: {}", line.len());

  let out1 = function_with_function_input(&|| false);
  println!("Output of function_with_function_input: {}", out1);

  let out2 = function_with_function_input_and_output(|| true);
  println!(
    "Output of function_with_function_input_and_output: {}",
    out2(10)
  );
}

fn function_with_function_input(g: &dyn Fn() -> bool) -> bool {
  let out = g();
  return out;
}

fn function_with_function_input_and_output(multiply: fn() -> bool) -> fn(i32) -> i32 {
  if multiply() {
    move |v| v * 2
  } else {
    move |v| v
  }
}

fn numbers() {
  let mut number = 33;

  increse(&mut number, 22);

  number += 11;

  println!("number: {}", number); // 66

  let simple_product = |x, y| x * y;
  let simple_product_advanced = |x: i32, y: i32| -> i32 { x * y };
  let out1 = product(number, 2);
  let out2 = simple_product(number, 2);
  let out3 = simple_product_advanced(number, 2);
  println!("{} {} {}", out1, out2, out3);
}

fn increse(val: &mut i32, by: i32) {
  *val += by
}

fn product(x: i32, y: i32) -> i32 {
  x * y
}
