enum SpreadSheepCell {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    let v2 = dbg!(vec![1, 2, 3, 4, 5]);
    let a1: [i32; 3] = dbg!([1, 2, 3]);

    let third: &i32 = &v2[2];
    println!("The value of the third element: {third}");

    let third: Option<&i32> = dbg!(v2.get(2));
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no element."),
    }

    let slice: &[i32] = dbg!(&v2[0..=2]);

    // let does_not_exist = dbg!(&v2[100]);
    let does_not_exist = dbg!(v2.get(100));

    let mut v3: Vec<i32> = vec!(1, 2, 3, 4, 5);

    let mut first: &i32 = &mut v3[0];

    // v3.push(6);

    println!("First elemnt of v3: {first}");

    let mut v: Vec<i32> = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    println!("{v:?}");

    let row: Vec<SpreadSheepCell> = vec![
        SpreadSheepCell::Text(String::from("Gilberto Ribeiro")),
        SpreadSheepCell::Int(32),
        SpreadSheepCell::Float(64.64),
    ];

    match &row[2] {
        SpreadSheepCell::Int(x) => println!("Inteiro: {x}."),
        SpreadSheepCell::Float(x) => println!("Float: {x}."),
        SpreadSheepCell::Text(x) => println!("Texto: {x}."),
    }
}
