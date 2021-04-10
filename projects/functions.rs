fn main() {
  //Testando expressões e declarações
  //Isso (linha 3) é uma declaração, esta linha não retorna valor e termina em ponto e vírgula
  let valor_somado = soma(1, 1);
  println!("O valor somado é: {}", valor_somado);

  //Uma variável pode receber um bloco de instruções para atribuir a ela um valor
  let _valor = {
    let valor_primario = 8;
    valor_primario + 2
  };
}

fn soma(x: u8, y: u8) -> u8 {
  //Já a linha 9 é uma expressão, retorna valor e não deve ser usado ponto e vírgula
  x + y
}
