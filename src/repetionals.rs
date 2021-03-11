fn main() {
  // Laço de repetição loop
  let mut valor = 1;
  loop {
    if valor == 5 {
      break;
    } else {
      println!("O tempo agora é {}", valor);
      valor += 1;
    }
  }

  //Laço de repetição while
  valor = 1
  while valor < 5 {
    println!("O tempo agora é {}", valor);
    valor += 1;
  }
}
