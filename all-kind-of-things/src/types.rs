enum Color {
  Red,
  Green,
  Blue,
  RGB(u8, u8, u8), // tuple
  Cmyk {
    cyan: u8,
    magenta: u8,
    yellow: u8,
    black: u8,
  }, // struct
}

pub fn run() {
  let colors = vec![
    Color::Red,
    Color::Blue,
    Color::Green,
    Color::RGB(20, 80, 45),
    Color::RGB(0, 0, 0),
    Color::RGB(255, 255, 255),
    Color::Cmyk {
      cyan: 150,
      magenta: 255,
      yellow: 0,
      black: 100,
    },
  ];

  for color in colors {
    match color {
      Color::Red => println!("Red"),
      Color::Green => println!("Green"),
      Color::Blue => println!("Blue"),
      Color::RGB(0, 0, 0) | Color::Cmyk { black: 255, .. } => println!("Black"),
      Color::RGB(255, 255, 255)
      | Color::Cmyk {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 0,
      } => println!("White"),
      Color::RGB(r, g, b) => println!("R={} G={} B={}", r, g, b),
      Color::Cmyk {
        cyan: c,
        magenta: m,
        yellow: y,
        black: b,
      } => println!("cyan={} magenta={} yellow={} black={}", c, m, y, b),
    }
  }

  uninonos();
  optionos();
  arrayos();
  tulpos();
  genericsos();
}

union IntOrFloat {
  i: i64,
  f: f64,
}

fn match_on_union(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42 } => {}
      IntOrFloat { i } => println!("Unproccessable int of float, assuming it's a int: {}", i),
    }
  }
}

fn uninonos() {
  let mut iof = IntOrFloat { i: 123 };
  iof.i = 1234;
  iof.f = 1.1; // This will break it whaaaa

  let value = unsafe { iof.i };
  println!("{}", value);

  match_on_union(iof);
}

fn test(a: f64, b: f64) -> Option<f64> {
  if a != 0.0 && b != 0.0 {
    Some(a / b)
  } else {
    None
  }
}

fn optionos() {
  let test_output = test(23.0, 2.0);
  let result = match test_output {
    Some(v) => v,
    None => 0.0,
  };
  println!("{}", result);

  if let Some(res) = test_output {
    println!("{}", res);
  } else {
    println!("trying to devide by a zero")
  }
}

fn reset_slice(slice: &mut [i32]) {
  println!("slice: {:?}, with lenght: {}", slice, slice.len());
  for i in 0..slice.len() {
    slice[i] = 0;
  }
}

fn arrayos() {
  let mut testje = [7, 6, 5, 4, 3];
  testje[0] = 1;
  println!("{:?} with a lenght of {}", testje, testje.len());

  let t2 = [2; 10]; // creates an array with 10 items with in every slot a 2
  println!("{:?}", t2);

  let t3 = [[1; 3]; 3]; // 2d matrix
  println!("{:?}", t3);

  let mut data = [5, 6, 7, 8, 9, 0];
  reset_slice(&mut data[1..4]);
  println!("{:?}", data);
  reset_slice(&mut data);
  println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
  (x + y, x * y)
}

fn tulpos() {
  let x = 3;
  let y = 4;
  let mut sp = sum_and_product(x, y);
  println!("sum: {}, product: {}", sp.0, sp.1);

  let inline_sum_and_product = |x: i32, y: i32| (x + y, x * y);
  sp = inline_sum_and_product(x, y);
  let (sum, product) = sp;
  println!("sum: {}, product: {}", sum, product);
}

struct Point<T> {
  x: T,
  y: T,
}

struct Line<T> {
  start: Point<T>,
  end: Point<T>,
}

struct AdvancedPoint<T, V> {
  x: T,
  y: V,
}

fn genericsos() {
  let a: Point<u16> = Point { x: 0, y: 0 };
  let b: Point<f64> = Point { x: 0.0, y: 0.0 };
  let c: AdvancedPoint<f32, i32> = AdvancedPoint { x: 1.0, y: 1 };
  let d = Point { x: 1f64, y: 1f64 };
  println!(
    "a: {{{}, {}}}, b: {{{}, {}}}, c: {{{}, {}}}",
    a.x, a.y, b.x, b.y, c.x, c.y
  );

  let line = Line { start: b, end: d };
  println!(
    "From: {{{}, {}}}, To: {{{}, {}}}",
    line.start.x, line.start.y, line.end.x, line.end.y
  );
}
