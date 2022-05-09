use colour::blue;

pub fn info(line: &str) {
  blue!("-> ");
  println!("{}", line);
}

