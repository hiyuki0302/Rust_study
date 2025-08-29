// ライフタイム(参照が指すデータが有効である期間)の目的はダンダリング参照(無効なメモリの参照)を回避すること。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    { // ここから
        let string2 = String::from("xyz"); 
        result = longest(string1.as_str(), string2.as_str());
    } // ここまでがxとyが両方とも有効な期間。ライフタイム'aになる。
    println!("The longest string is {}", result); // ライフタイム'aが既に終わっているのに、resultを呼ぼうとしているのでエラーになる。
}

// 'aがライフタイム。x,y,strは同じ有効期間ですよというのを教えてあげるためのもの。
// ライフタイム'aはxとyが指すデータが2つとも有効な最も短い期間（共通部分）になる。
// 静的ライフタイム→'static 全期間生存できる。

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    //       "アナウンス！ {}"
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
