〇一般的なコレクション
☆ベクタ型
同じ型しか保存できない。

-作成方法-
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // マクロを使った場合
コンパイラはどんな値を保持するか予測できないので型注釈が必要。

-値の更新方法-
let mut v = Vec::new();
v.push(5);
v.push(6);
可変にしてpush。pushされ、配置された値から型を推論してくれるので型注釈は不要。

-破棄-
スコープを抜けたら破棄される。中身も消える。

-値の取得について-
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // 方法1
println!("The third element is {}", third);

match v.get(2) { // 方法2
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

・範囲外の指定
let v = vec![1, 2, 3, 4, 5];
let does_not_exist = &v[100]; // クラッシュ
let does_not_exist = v.get(100); // Noneを返す

・不変借用していると同一スコープ内においては値の変更ができない。
・ループでベクタ要素への可変参照
可変参照が参照している値を変更するには、参照外し演算子(*)を使用して、参照外しを行う必要がある
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; 
}

・Enumを使って複数の型を保持
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

---------------------------------------------------------------------------------------------
☆文字列
Rustには、言語の核として1種類しか文字列型が存在しない。
文字列スライスのstrで、通常借用された形態&strでよく用いられる。
よくあるString型はRustの標準ライブラリで提供される。伸長可能であり、可変、所有権のあるUTF-8エンコードされた文字列型。
文字列と指したら大体Stringか&strのことを言ってると思えよ～

-新規文字列を生成する-
let mut s = String::new();
新しい空のsという文字列を生成。

let data = "initial contents";
let s = data.to_string();
↑↓　この2つのコードの意味は同じ。文字列リテラルから文字列を生成してくれる。
let s = String::from("initial contents");

-文字列の更新-
Stringはサイズを伸ばすことができる。
・push_strとpush
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
→所有権がなければ本来出力できないが、push_strならできる。

let mut s = String::from("lo");
s.push('l');
→lol

・＋演算子、Formatマクロ
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意

+演算子は以下の関数メソッドを使用している。
fn add(self, s: &str) -> String {

formatマクロを利用すると以下のようになる。引数の所有権を奪わない。
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);

・文字列をスライス
文字列の境界を理解する必要がある。
let hello = "Здравствуйте";
let s = &hello[0..4];
これらの文字は各々が2バイトになるため、sはЗдになる。
&hello[0..1]にしたらパニックが起きRustが永眠します。
また、Unicodeのスカラー値は2バイト以上のものもあるので、スライスする際は気を付けよう。

---------------------------------------------------------------------------------------------
☆ハッシュマップ
HashMap<K, V>はK型のキーとV型の値の対応関係を保持する。データはヒープに保存される。
.isnertで値を追加できる。

-生成-
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);


use std::collections::HashMap;
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
→Blueと10がペアになる。collectメソッドを使ってタプルのベクタをハッシュマップに変換することができる。

-アクセス-
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

-上書き-
use std::collections::HashMap;
 
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
// {"Blue": 25}

-キーに値がなかった場合に挿入-
entryという特別なAPIがあり、これは引数としてチェックしたいキーを取る。
entryメソッドの戻り値は、Entryといわれるenumであり、存在したりしなかったりする可能性のある値を表す。

use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50); 
// socresというハッシュマップにあるYellowキーに紐づいている値はあるか。
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
