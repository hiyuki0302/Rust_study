〇モジュールの呼び出し
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
    super::su

    // Relative path
    // 相対パス
    front_of_house::hosting::add_to_waitlist();
}

rustにおけるあらゆるの要素は標準的には非公開になっている。
そのため相対パス、絶対パスによる呼び出しが必要。
pubをつけることで公開することができる。上記のコードだと、pubをmod hostingとfn add_to_waitlistにつけることで別の関数からアクセスできるようになる。
crate→クレートのルート（最上位）を参照
super→現在のモジュールの親モジュール（１つ上）を参照
self→現在のモジュールを参照

---------------------------------------------------------------------------------------------
〇構造体とenumの公開
mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // 公開
        seasonal_fruit: String, // 非公開
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // 季節のフルーツは公開されていないので変更できない。
}

Structだと全てのフィールドにpubを付けないといけないが、enumだと頭にpubをつけることでヴァリアントすべてが公開される。
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

---------------------------------------------------------------------------------------------
〇useキーワードでパスをスコープに持ち込む
頭にuse付けて相対パスを付けたらいい。下のコードのように呼び出すことができる。
mod test_of_test {
    pub mod test{
        pub fn fuck_of_you() {
        }
    }
}

use crate::test_of_test::test; // 絶対パス
use self::test_of_test::test; // 相対パス

pub fn eat_test( {
    test::fuck_of_you();
})

・enumや構造体、そのほかの要素を持ち込むときはフルパスで記述するのが慣例的。
・同じ名前の２つの型をスコープに持ち込むことはできない。
解決策はasを利用してリネーム、親モジュールを使って区別するなどの方法がある。
・pub useを使うことで外部のコードが呼び出せるようにできる。
例：pub use crate::...::...;

---------------------------------------------------------------------------------------------
〇ネストしてパス
use std::cmp::Ordering;
use std::io;
↓
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
↓
use std::{self, Write};

---------------------------------------------------------------------------------------------
〇glob演算子
・パスにおいて定義されているすべての公開要素をスコープに持ち込みたい場合。
use std::io::collections::*;

---------------------------------------------------------------------------------------------
〇モジュールを複数のファイルに分割する
my-projectフォルダ参照。
pub mod hosting;
このようにモジュールのあとにブロックではなくセミコロンを入れることで、モジュールの名前と同じ名前のファイルを読み取ってくれる。