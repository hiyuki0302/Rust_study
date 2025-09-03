〇構造・フィールド
・フィールド・・・データ辺の名前と型を制御する。
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com")
インスタンスの値を変更する場合はドットを使う。

省略した書き方
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

・構造体更新記法
インスタンスのフィールドを明示的に指定しない場合、以下のように書くことができる。
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

・タプル構造体
Structの後に構造体名、型式を指定して定義する。
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

例：
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {　// &を付けて借用にしないと、関数areaが所有権を持ち、main()で利用できなくなる。
    rectangle.width * rectangle.height
}

・構造体の場合、println!が利用できない。println!がどのように整形して出力すればいいかわからないから。
#[derive(Debug)]を構造体定義の直上につけることででバックを行うことができる。

〇メソッド記法
最初の引数は必ずself!

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {  //impl(実装ブロック)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {   // other は引数
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }　// 関連関数。
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(3); // 関連関数の呼び出し

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // selfはrect1の値を参照する。
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}