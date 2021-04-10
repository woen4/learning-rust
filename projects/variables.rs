fn main() {
    // Atribuição de valores a variáveis
    // Por padrão toda variável é imutável
    let string = "Isso é uma variável imutável";
    println!("{}", string);

    // Atribuição de valores a varáveis mutáveis (mut)
    // O endereço de memória é o mesmo, a variável é a mesma
    let mut value = 5;
    println!("O valor é {}", value);
    value = 10;
    println!("O novo valor é {}", value);

    /*
        Declarando constantes

        Variáveis por padrão são imutáveis, mas como já vimos
        isso pode ser alterado usando o mut, entretanto quando queremos declarar
        constantes, usaremos o const
    */
    //O padrão da comunidade é uppercase + snake_case
    const CONSTANTE: i32 = 5;
    println!("Minha constante: {}", CONSTANTE);
}
