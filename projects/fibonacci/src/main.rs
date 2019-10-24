use std::io;

fn main() {
  println!("Hello, which Fibonacci number would you like to find?");

  let mut index = String::new();

  io::stdin().read_line(&mut index)
    .expect("Failed to read line");

  let index: u32 = match index.trim().parse() {
    Ok(num) => num,
    Err(_) => 0,
  };

  println!("Answer is {}", find_fibonacci_number(index)); 
}

fn find_fibonacci_number(x: u32) -> u32 {
  let mut count = 1;
  let mut previous_count = 0;

  for index in 1..x {
    let temp_count = count;
    count += previous_count;

    previous_count = temp_count;
  }
  
  count
}
