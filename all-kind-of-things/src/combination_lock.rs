use std::io::stdin;

enum State {
  Locked,
  Failed,
  Unlocked,
}

pub fn lock() {
  let code = String::from("1234");
  let mut state = State::Locked;

  loop {
    match state {
      State::Locked => {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
          Ok(_) => {
            if input.trim_end() == code {
              state = State::Unlocked;
              continue;
            } else {
              state = State::Failed;
              continue;
            }
          }
          Err(_) => continue,
        }
      }
      State::Failed => {
        println!("FAILED");
        state = State::Locked;
      }
      State::Unlocked => {
        println!("UNLOCKED");
        return;
      }
    }
  }
}
