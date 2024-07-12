fn main() {

    // matriz();

    // println!("Fim de semana? {}", eh_fim_de_semana_2(DiaDaSemana::Domingo));

    let cor: Cor = Cor::CymkColor{cyan: 25, yellow: 25, magenta: 25, black: 255};

    println!("A cor é {}.", cores(cor))
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
        Cor::RgbColor(_, _, _) => String::from("RGB desconhecido"),
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