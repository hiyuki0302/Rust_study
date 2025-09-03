〇列挙子
プログラムが遭遇しうるすべての可能性を列挙することができる。
例：IPアドレスは2種類ある。Ipv4かIpv6。どちらかにはなるが両方にはなりえない。コードがどちらの種類になっても扱う際には同じ型として扱われるべき。
すべてのインスタンスが同じフィールド構造を持つ場合は構造体を選択するべきだが、使わないフィールドがある場合などはEnumが適している。
この概念をコードで列挙型として定義し、Ipアドレスがなりうる種類を列挙することで表現できる。
enum IpAddrKind {
    V4,
    V6,
}
これでほかの場所で使用できる独自のデータ型になる。

・インスタンス生成
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_type: IpAddrKind) {}
route(IpAddrKind::V4);
route(IpAddrKind::V6);

・データ保持
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
これを
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
こう書ける。余計な構造体Structを書く必要がない。
---------------------------------------------------------------------------------------------
標準ライブラリにあるIpAddrKindに対する定義があるので以下のように書くこともできる。
struct Ipv4Addr {
    // 省略
}
struct Ipv6Addr {
    // 省略
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
---------------------------------------------------------------------------------------------
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

↑↓は同様の意味合いを持つ。enumの中には文字列や数値型、構造体などいかなる種類のデータも格納することができる。

struct QuitMessage; // ユニット構造体
struct MoveMessage {x: i32, y: i32,}
struct WriteMessage(String); // タプル構造体
struct ChangeColorMessage(i32, i32, i32); // タプル構造体
---------------------------------------------------------------------------------------------
以下のコードのようにenumの中にはメソッドを入れることもできる。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
---------------------------------------------------------------------------------------------
・Option enum
Rustにはnullがない。しかし、値が存在するか不在かという概念をコード化するEnumがある。→Option<T>
enum option<T> {
    Some<T>,
    None,
}
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
このコードはコンパイルエラーになる。
i8とOption<i8>は異なる型なのに足そうとしているから。
---------------------------------------------------------------------------------------------
〇match制御フロー演算子
マッチしたパターンに応じて処理をしてくれる制御フロー演算子。
ifでは論理値を返す必要があるが、matchではどんな型でも構わない。

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // stateという変数をパターンに追加。Coin::Quarterがマッチすると、stateに変数が束縛される。
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
---------------------------------------------------------------------------------------------
〇Option<T>とのマッチ
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Noneを入れないとコンパイルエラーになる。すべての可能性を考慮しないといけない。
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

マッチは包括的。

・_プレースホルダー
_を使うことでそれ以外、というmatchに条件を課すことができる。
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}

---------------------------------------------------------------------------------------------
〇if let
・match_ver
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("helloooooo"),
    _ => (), // Some(3)ではないときの処理も書く必要がある。
};

・if let_ver
if let Some(3) = some_u8_value {
    println!("three");
}else{ // elseも入れれる。
    count += 1;
}

Some(3)にマッチしたときのみ、なにかするというコード。
matchでも書けるが、ちょっと長いよね。

