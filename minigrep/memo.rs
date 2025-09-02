use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // argsの型を明示的に注釈して文字列のベクタを指定。Collectは型注釈が必要。
    println!("{:?}", args);
}
// コマンドラインに引数を渡さない場合、["target\\debug\\minigrep.exe"]と出力される。
// 最初の要素args[0]は、常に実行可能ファイル自体のパスになるため。

-----------------------------------------------------------------------
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // コマンドラインの第1引数
    let filename = &args[2];// コマンドラインの第2引数

    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);
}

-----------------------------------------------------------------------

