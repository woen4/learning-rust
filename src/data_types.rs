fn main() {
  //Tipagens
  //Rust tem tipagem estática isso indica que precisamos declarar o tipo das variáveis

  let _string = 2; //Aqui eu não precisei porque o compilador já infere que isso

  /* Tipo inteiro */
  let _number8_min_signed: i8 = -128;
  let _number8_max_signed: i8 = 127;
  //
  let _number8_min_unsigned: u8 = 0;
  let _number8_max_unsigned: u8 = 255;

  //A diferença de unsigned para signed é que o signed aceita valores negativos e positivos
  //Já o unsigned guarda apenas valores absolutos

  /*
    O tamanho máximo do signed é sempre o a metade do unsigned

    Pois se um valor que armazena 8 bits armazenar apenas valores absolutos
    ele armazenará do 0 ao 255 -> 256 valores -> 2^8 -> unsigned

    Já no signed ele armazenará ->
    -128 ao 127 -> 256 valores -> 2^8
    O signed além de guardar um valor precisa guardar o oposto dele
  */

  /*
  8-bit	i8	u8
  16-bit	i16	u16
  32-bit	i32	u32
  64-bit	i64	u64
  arch	isize	usize -> leva em conta a arquitetura do sistema
                        se for um sistema x64 o tamanho será de 64bits
                        se for um sistema x86 o tamanho será de 32bits
  */

  /* Ponto Flutuante => float */

  let _float_simple_precision: f32 = 5.112;
  let _float_double_precision: f64 = 5.1123123129;

  /* Operações aritméticas */
  // soma
  let _soma = 5 + 10;

  // subtração
  let _diferenca = 95.5 - 4.3;

  // multiplicação
  let _produto = 4 * 30;

  // divisão
  let _quociente = 56.7 / 32.2;

  // resto
  let _resto = 43 % 5;

  /* Booleano */
  let _f: bool = false;

  /* Caractere (Deve estar em aspas simples e o tamanho máximo é de 1 caractere) */
  let _caractere: char = 'a';

  // --> Tipos compostos

  /* Tuplas */
  // Assemelha-se a uma array do javascript, no entanto os valores não tem tipo constante
  let tupla: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tupla;

  println!("Valores na tupla: x:{} y:{} z:{}", x, y, z);

  //Acessando valores da tupla pelo index

  let _quinhentos = tupla.0;
  let _seis_quatro = tupla.1;
  let _um = tupla.2;

  /* Matriz */
  // Uma matriz no rust sempre tem valor constante, ela não pode crescer
  // em número de elementos, (Matrizes são armazenadas na stack memory)
  let a = [1, 2, 3, 4, 5];
}
