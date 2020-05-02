fn print_vec1(x: Vec<i32>) -> Vec<i32> {
  println!("{:?}", x);
  x
}

fn print_vec2(x: &Vec<i32>) {
  println!("{:?}", x);
}

struct Person {
  name: String,
}

struct Company<'z> {
  name: String,
  ceo: &'z Person,
}

pub fn run() {
  let v1 = vec![1, 2, 3];
  let v2 = print_vec1(v1);
  print_vec2(&v2);
  println!("{:?}", v2);

  let mut a = 40;
  {
    let b = &mut a;
    *b += 2;
  }
  let c = &mut a;
  *c += 2;
  drop(c);

  println!("{}", a);

  let boss = Person {
    name: String::from("Bill gates"),
  };
  let company = Company {
    name: String::from("Microsoft"),
    ceo: &boss,
  };
  println!("{}", boss.name);
  println!("{}", company.name);
  println!("{}", company.ceo.name);
}
