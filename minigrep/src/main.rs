extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| { // unwrap_or_elseはResult＜T,E＞に定義されている。
        // 引数解析時に問題
        eprintln!("Problem parsing arguments: {}", err); // エラーを標準出力ではなく。標準エラーに出力するように
        process::exit(1); // プログラムの停止
    });

    if let Err(e) = minigrep::run(config) { // runがErrを返しているか。フルパスで呼び出している。
        // この関数はエラー検知が目的のため、Okの場合なにも返す必要がない。
        println!("Application error: {}", e);
        process::exit(1);
    }
}


