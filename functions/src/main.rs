fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する
}
// 文字列に追加したい場合はString型。let mut s = "hello";だと中身ごとしか変えられない。
// String型は可変のため、コンパイル時に不明な量のメモリをヒープに確保してくれる。
