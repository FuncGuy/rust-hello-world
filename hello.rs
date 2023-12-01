fn print_number(number: &mut i32) {
  *number += 1;
  println!("The number is: {}", number);
}

fn main() {
  let mut x = 42;
  print_number(&mut x);  // Taking an immutable reference to x
}