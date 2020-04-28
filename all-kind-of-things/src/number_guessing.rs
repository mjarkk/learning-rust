use rand::Rng;
use std::io::stdin;

pub fn run() {
  let num = rand::thread_rng().gen_range(1, 101);

  loop {
    let mut input = String::new();
    if let Err(_) = stdin().read_line(&mut input) {
      break;
    };

    let input_number: i64 = match input.trim_end().to_string().parse() {
      Ok(v) => v,
      Err(_) => {
        println!("Input not a number using 0");
        0
      }
    };
    match input_number {
      v if v > num => println!("To hight"),
      v if v < num => println!("To low"),
      _ => {
        println!("Correct!!");
        break;
      }
    }
  }
}
