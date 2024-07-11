fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 7.0, 9.0];

    for nota in notas {
    println!("Nota: {}", nota);
    }

    println!();

    for i in 0..notas.len() {
        println!("Nota {}: {}", i, notas[i]);
    }

    println!();

    matriz();
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
    }
}