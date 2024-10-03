use closures;

use std::thread;

fn main() {

    // closures::generate_workout(10, 7);
    // move_between_threads();
    teste_fn_traits();
    sort();

}

fn closure_annotation() {
    fn add_one_v1(x: u32) -> u32 {x + 1}
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    let a = add_one_v3(5i32);
    let b = add_one_v4(5i16);
    let c = add_one_v2(7u32);
}

fn fn_example_closure() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

fn borrow_imut() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn borrow_mut() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("After defining closure: {list:?}");
    
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// fn move_between_threads() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(|| println!("From thread: {list:?}"))
//         .join()
//         .unwrap();

//     // println!("After moving to new thread: {:?}", list)
// }

fn teste_fn_traits() {
    let mut teste = String::from("Teste: ");
    let mut concatenar = |palavra| {
        teste.push_str("   ");
        format!("{teste}{palavra}")
    };
    let teste = String::from("TESTE: ");
    println!("{}", concatenar(String::from("Gilberto")));
}

fn sort() {
    #[derive(Debug)]
    struct Rectangle<T> {
        height: T,
        width: T,
    }

    impl<T> Rectangle<T> {
        fn new(width: T, height: T) -> Self {
            Self {
                width,
                height,
            }
        }
    }

    let mut list: [Rectangle<u32>; 4] = [
        Rectangle::new(5, 6),
        Rectangle::new(8, 9),
        Rectangle::new(2, 4),
        Rectangle::new(1, 9),
    ];

    list.sort_by_key(|x| x.width);

    println!("{:?}", list);
}