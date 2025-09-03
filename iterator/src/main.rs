pub trait Iterator {
    type Item; 
    // Type Itemは関連型。Itemとしてイテレータが生成する要素の型を指定する必要がある。

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1 = vec![2, 5, 1];

    let mut v1_iter = v1.iter(); // Item = &i32

    println!("{:?}", v1_iter.next()); // Some(2)
    println!("{:?}", v1_iter.next()); // Some(5)
    println!("{:?}", v1_iter.next()); // Some(1)
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

