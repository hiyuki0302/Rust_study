pub trait Iterator {
    type Item; 
    // Type Itemは関連型。Itemとしてイテレータが生成する要素の型を指定する必要がある。

    fn next(&mut self) -> Option<Self::Item>; 
    // Iteratorトレイトを実装する時は、必ずNextメソッドを実装する必要がある。Nextを呼び出すメソッドは消費アダプタと呼ばれる。
    // ていうか使わないとIteratorを使う意味がないンゴよ～
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

-----------------------------------------------------------------------------------------
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter() // ベクタの所有権を奪う
        .filter(|s| s.size == shoe_size) 
        // |s|→&Shoe sはなんでもいい。自分で決めれる。
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
