fn main() {
  let valor_carteira = 5;

  //Condicionais no rust são como no javascript mas aqui não se coloca parenteses
  if valor_carteira == 0 {
    println!("A carteira está vazia");
  } else if valor_carteira == 5 {
    println!("A carteira está meio cheia");
  } else {
    println!("A carteira está ?")
  }

  // Atribuindo valores a variáveis a partir de um if
  let status = if valor_carteira > 0 { "ok" } else { "pobre" };
  println!("O status é {}", status);
}
