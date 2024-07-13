fn main() {

    // matriz();

    // println!("Fim de semana? {}", eh_fim_de_semana_2(DiaDaSemana::Domingo));

    // let cor: Cor = Cor::RgbColor(12, 25, 68);

    // println!("A cor é {}.", cores(cor))

    // consulta_opcional();

    // vectors();

    // vectors_with_capacity();

    conta_corrente();

}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia: u8) -> bool {
    match dia {
        0 | 6 => true,
        _ => false
    }
}

fn eh_fim_de_semana_2(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

#[allow(dead_code)]
enum Cor {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, yellow: u8, magenta: u8, black: u8}
}

fn cores(cor: Cor) -> String {
    match cor {
        Cor::Red | Cor::RgbColor(255, 0, 0) => String::from("vermelho"),
        Cor::Green | Cor::RgbColor(0, 255, 0) => String::from("verde"),
        Cor::Blue | Cor::RgbColor(0, 0, 255) => String::from("azul"),
        Cor::RgbColor(0, 0, 0)
            | Cor::CymkColor{cyan: _, yellow: _, magenta: _, black: 255} => String::from("preto"),
        Cor::RgbColor(255, 255, 255) => String::from("branco"),
        Cor::RgbColor(red, green, _) => {
            println!("Vermelho {} e verde {}", red, green);
            String::from("RGB desconhecido")
            },
        Cor::CymkColor{cyan: _, yellow: _, magenta: _, black: _} => String::from("CYMK desconhecido")
    }
}

fn matriz() {
    let m: [[f32; 3]; 2] = [
        [3.14; 3],
        [5.0; 3]
    ];

    for i in 0..2 {
        for j in 0..3 {
            println!("M[{i},{j}] = {}", m[i][j])
        }
    }let notas: [f32; 4] = [10.0, 8.0, 7.0, 9.0];
    let inteiro: usize = 2;

    for nota in notas {
    println!("Nota: {}", nota);
    }

    println!();

    for i in 0..notas.len() {
        println!("Nota {}: {}", i, notas[i]);
    }

    println!();
    println!("Nota {}: {}", inteiro, notas[inteiro]);
    println!();
}

fn lista_de_notas() {
    let notas: [f32; 4] = [10.0, 8.0, 7.0, 9.0];
    let inteiro: usize = 2;

    for nota in notas {
    println!("Nota: {}", nota);
    }

    println!();

    for i in 0..notas.len() {
        println!("Nota {}: {}", i, notas[i]);
    }

    println!();
    println!("Nota {}: {}", inteiro, notas[inteiro]);
    println!();
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from("/home/Downloads"));

    // match &conteudo_arquivo {
    //     Some(valor) => println!("{}", valor),
    //     None => println!("Falha na consulta")
    // }

    println!("{:?}", conteudo_arquivo);

// Não entendi muito esse if let
    if let Some(valor) = conteudo_arquivo {
        println!("Resultado da consulta: {}", valor);
    }
}

fn ler_arquivo(_caminho: String) -> Option<String> {
    Some(String::from("Alguma consulta."))
}

// Generics

// enum Option<T> {
//     Some(T),
//     None
// }

// enum Result<S, E> {
//     Ok(S),
//     Err(E)
// }

fn vectors() {
    // let mut notas: Vec<f32> = Vec::new();
    let mut notas: Vec<f32> = vec![9.9, 6.5, 8.7];
    let i: usize = 2;
    notas.push(10.0);
    println!("{:?}", notas);
    println!("{}", notas[1]);
    println!("Nota {}: {}.", i, match notas.get(i-1) {
        Some(n) => *n,
        None => 0.0
    });
    println!("{:?}", notas.get(i-1));
    println!("Tamanho de notas: {}", notas.len());
    for i in 0..notas.len() {
        println!("{}", notas[i]);
    }

    if let Some(n) = notas.get(i-1) {
        println!("if let: {}", *n);
    }

    // while let Some(n) = notas.pop() {
    //     println!("{}", n);
    // }

    println!("{:?}", notas);

    for nota in &notas {
        println!("{}", nota);
    }

    println!("{:?}", notas);
}

fn vectors_with_capacity() {
    let mut notas: Vec<f32> = Vec::new();
    notas.push(5.0);
    notas.push(6.5);
    println!("{:?}", notas);
    println!("Tamanho: {}. Capacidade: {}.", notas.len(), notas.capacity());
    notas.push(6.7);
    notas.push(8.9);
    notas.push(7.2);
    println!("{:?}", notas);
    println!("Tamanho: {}. Capacidade: {}.", notas.len(), notas.capacity());
    notas.push(2.3);
    println!("{:?}", notas);
    println!("Tamanho: {}. Capacidade: {}.", notas.len(), notas.capacity());
}

struct Conta {
    titular: Titular,
    saldo: f32
}

impl Conta {
    fn sacar(&mut self, valor: f32) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente() {
    let titular_1: Titular = Titular { nome: String::from("Gilberto"), sobrenome: String::from("Ribeiro") };
    let mut conta_1: Conta = Conta {
        titular: titular_1,
        saldo: 100.0
    };

    conta_1.sacar(20.0);
    println!("Titular: {} {}.\nSaldo: {}.", conta_1.titular.nome, conta_1.titular.sobrenome, conta_1.saldo);
}
