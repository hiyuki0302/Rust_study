use std::io; //stdという標準ライブラリからioという入出力ライブラリをスコープに入れる。
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is :{}", secret_number);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed tor read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// !がつくとマクロ
// デフォルトでは変数は不変だが、mut(mutable)をつけることによって下辺にすることができる。
// String::new　文字列（String）の新しい空の文字列というインスタンス
// &mut guess 頭に「&」つけると参照。mut guess を参照しますとのこと。
// let guess: u32のようにコロンをつけることで型の注釈を入れている。
// trimメソッドは空白や改行(\n)を削除してくれる。
// nado
