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

