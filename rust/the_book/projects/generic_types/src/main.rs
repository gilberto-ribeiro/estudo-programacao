fn main() {
    let number_list = vec![1, 5, 60, 45, 100, 20, 30];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = find_largest_i32(&number_list);
    println!("Largest number: {largest}");

    let char_list = vec!['a', 'b', 'j', 'f', 'u', 'c', 'g'];
    let largest = find_largest_char(&char_list);
    println!("Largest char: {largest}");

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1f64, y: 2.0};
    let float_2 = Point { x: 3.0, y: 5.0};
    println!("Integer: x={} y={}", integer.x(), integer.y);
    println!("Float: x={} y={}", float.x, float.y());
    println!("Distance from origin: {}", float.distance_from_origin());
    println!("Distance from point 1 to 2: {}", float.distance_from_other_point(&float_2));

    let p1 = NewPoint{x: 5, y: 10.4};
    let p2 = NewPoint {x: 'a', y: "Gilberto"};
    println!("P1: {} {}", p1.x, p1.y);
    println!("P2: {} {}", p2.x, p2.y);
    let p3 = p1.mixup(p2);
    println!("P3: {} {}", p3.x, p3.y);
    let integer = Some(5i32);
    let float = Some(10f64);
    println!("{} {}", integer.unwrap(), float.unwrap());
}

fn find_largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn find_largest<T>(list: &[T]) -> &T {
//     let mut largest: &T = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn distance_from_other_point(&self, another_point: &Point<f64>) -> f64 {
        ((self.x - another_point.x).powi(2) + (self.y - another_point.y).powi(2)).sqrt()
    }
}

struct NewPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> NewPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: NewPoint<X2, Y2>) -> NewPoint<X1, Y2> {
        NewPoint {
            x: self.x,
            y: other.y,
        }
    }
}
