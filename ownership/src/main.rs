fn main() {
    let str = String::from("Ola mundo");
    let inteiro: u8 = 2;
    //Ao passar uma variável não Copy (Que é guardada na heap) para uma função
    //a variável se invalida, perde seu escopo

    //A variável aqui é copiada e passada na função mas como vimos
    //Variáveis não copy não podem ser copiadas diretamente, pois se invalidam
    toma_posse(str);
    //Error -> println!("{}", str);

    //A variável aqui é copiada e passada na função
    toma_posse_de_variavel_copy(inteiro);
    println!("{}", inteiro);
}

fn toma_posse(str: String){
    println!("{}", str);
}

fn toma_posse_de_variavel_copy(inteiro: u8){
    println!("{}", inteiro);
}

//Mas e quando queremos trabalhar com uma variável sem passar a pose dela?