fn main() {
  const POSITION_WANTED: u8 = 50;
  let result = fibonacci(POSITION_WANTED);
  println!("Result: {}", result);
}

fn fibonacci(position: u8) -> u32 {
  const INITAL_ARRAY_LENGTH: u8 = 2;
  let mut sequence = vec![0, 1];
  let number_target: u8 = position - INITAL_ARRAY_LENGTH;

  for _i in 0..number_target {
    let current_length = sequence.len();
    let next_value = sequence[current_length - 1] + sequence[current_length - 2];
    sequence.push(next_value);
  }

  let last_position_of_array = sequence.len() - 1;
  sequence[last_position_of_array]
}
