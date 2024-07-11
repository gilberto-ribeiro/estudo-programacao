const PI:f32 = 3.14159265358;
static mut GLOBAL:f32 = 1.;

fn soma(a:f32, b:f32) -> f32 {
//    println!("LALALA: {}, {}, {}", a, b, a-b);
    return a + b
}

fn ownership() {
    let mut uma_string = String::from("Gilberto");
    println!("{}", uma_string);
    rouba(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba(string:&mut String) {
    string.push_str(" Ribeiro");
    println!("String: {}.", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}.", x, match x {
            1 => "Muito pouco",
            2 | 3 => "Pouco",
            4..=10 => "Razoável",
            _ if x % 2 == 0 => "Um bocado par",
            _ => "Muito"
        });
    }
}

fn sombra() {
    let a:i32 = 50;

    {
        println!("fora-dentro, a: {}.", a);
        let a:i32 = 100;
        let b:i32 = 200;
        println!("dentro, a: {}.", a);
        println!("dentro, b: {}.", b);
    }
    println!("fora, a: {}.", a);
//    println!("fora, b: {}.", b);
}

fn escopo() {
    
    let variavel:i32 = 300;
    println!("Variável = {}. Tamanho da variável: {} bytes.", variavel, std::mem::size_of_val(&variavel));

    let mut variavel:i32 = 400;
    variavel = variavel + 200;
    println!("Novo valor da variável: {}.", variavel);

    let booleana:bool = true;
    println!("O valor da booleana é: {}. Tamanho da boobleana: {} bytes.", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'A';
    println!("Letra {}. Tamanho da letra é {} bytes.", letra, std::mem::size_of_val(&letra));

    println!("O valor de pi é: {}.", PI);
    unsafe {
        GLOBAL = 2.0;
        println!("O valor da variável global é: {}.", GLOBAL);
    }
}

fn main() {
//    escopo();
//    sombra();
//    let a:f32 = 3.0;
//    let b:f32 = 5.0;
//    println!("A soma de {} e {} é {}.", a, b, soma(a, b));
//    println!("Novo valor da variável: {}.", variavel);

//    condicionais(15);

//    repeticoes(5);
//
//    let nome = "Nazi";
//    let sobrenome = match nome {
//        "Gilberto" => "Ribeiro",
//        "Iago" => "Aguiar",
//        "Nazi" => "Gui",
//        _ => "NA"
//    };
//
//    println!();
//    println!("Sou {} {}.", nome, sobrenome);

//    ownership();

//    pattern_matching();

    erros();
}

fn condicionais(idade:u8) {

    let responsavel_autorizou:bool = false;

    let maior_de_idade:bool = idade >= 18;

    if maior_de_idade {
        println!("É maior de idade.");
    }
    else {
        println!("É menor de idade.");
    }

    let condicao;

    condicao = if maior_de_idade { "maior" } else { "menor" };
    println!("A pessoa é {} de idade.", condicao);
}

fn repeticoes(multiplicador:u16) {

    let mut contador:u16 = 0;
    let mut produto:u16;

    println!("Uso do while");
    while contador < 10 {
        contador += 1;
        produto = multiplicador * contador;
        println!("{} x {} = {}", multiplicador, contador, produto);
    }

    contador = 0;

    println!();
    println!("Uso do loop");
    loop {
        contador += 1;
        if contador == 3 || contador == 5 {
            continue;
        }
        produto = multiplicador * contador;
        println!("{} x {} = {}", multiplicador, contador, produto);
        if contador >= 10 {
            break;
        }
    }

    println!();
    println!("Uso do for");
    for i in 1..=10 {
        if i == 7 {
            continue;
        }
        produto = i * multiplicador;
        println!("{} x {} = {}", multiplicador, i, produto);
    }
}

fn erros() {
//    let v = vec![1, 2, 3];
//    println!("{}", v[3]);
//    panic!("Erro proposital!")

    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Código de erro = {}", numero)
    };
}

fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo certo."))
//    Err(42)
}
