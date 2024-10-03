fn main() {
    basic();
}

fn basic() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let v_iter = v.iter();
    for value in v_iter {
        println!("Value {}", value);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v = vec![1, 3, 5, 7, 9];
    let mut s = v.iter().map(|x| x * x);
    assert_eq!(s.next(), Some(1));
    assert_eq!(s.next(), Some(9));
    assert_eq!(s.next(), Some(25));
    assert_eq!(s.next(), Some(49));
    assert_eq!(s.next(), Some(81));
    assert_eq!(s.next(), None);
}

#[test]
fn iterator_map_2() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).map(|y| y * y).collect();
    assert_eq!(v2, vec![4, 9, 16]);
}
