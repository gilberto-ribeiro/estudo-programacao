enum SpreadSheepCell {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    strings()
}

fn vectors() {
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

fn strings() {
    let mut s1: String = String::from("Hello");
    let s2: String = "World".to_string();
    println!("{s1}, {s2}.");
    s1.push_str(", World...");
    println!("Push_str: {s1}");
    s1.push_str(&s2);
    println!("Push_str: {s1}");
    let s3 = s1 + ", " + &s2;
    println!("{s3}");

    let mut t1 = String::new();
    t1.push_str("tic");
    let t2 = String::from("tac");
    let t3 = "toe".to_string();
    let t4 = format!("{t1} - {t2} - {t3}");
    let t5 = t1 + "-" + &t2 + "-" + &t3;

    println!("t4: {t4}");
    println!("t5: {t5}");

    let n1: &str = "aeiou";
    let n2: String = String::from(n1);
    println!("{n1} - {n2}");
    // println!("{}", &n1[0]);
    // println!("{}", n2[0]);

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}");

    println!("Imprimindo os chars:");
    for c in s.chars() {
        println!("{c}");
    }
    println!("Imprimindo os bytes:");
    for b in s.bytes() {
        println!("{b}");
    }
    
}