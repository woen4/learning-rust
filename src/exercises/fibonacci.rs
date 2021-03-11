fn main() {
  const POSITION_WANTED: u16 = 50;
  let result = fibonacci(POSITION_WANTED);
  println!("Result: {}", result);
}

fn fibonacci(position: u16) -> i32 {
  let number_target = position - 2;
  let mut sequence = Vec::new();
  sequence.push(0);
  sequence.push(1);
  for _i in 0..number_target {
    let sequence_len = sequence.len();
    let next_value = sequence[sequence_len - 1] + sequence[sequence_len - 2];
    sequence.push(next_value);
  }
  sequence[sequence.len() - 1]
}
