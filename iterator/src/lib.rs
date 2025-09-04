use std::error::Error;
use std::fs::File;
use std::io::prelude::*; // 入出力処理
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // コンソールから入力された値をイテレータにし、1つずつ取り出す。env::argsの戻り値の1つ目はプログラム名なので、まず1回Next
        args.next(); 
        let query = match args.next() {
            Some(arg) => arg,
            // クエリ文字列を取得しませんでした
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            // ファイル名を取得しませんでした
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // Boxはトレイトオブジェクト。dynはDynamicの略。
    let mut f = File::open(config.filename)?; // ?演算子をつけることで、エラー時にErr(error)を返してmain関数を終了

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) // Trueの場合
    } else {
        search_case_insensitive(&config.query, &contents) // Falseの場合
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) 
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // 新しいデータを作成することになるので、Stringになる。
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // &を付けることで文字スライスに変換できる。(str型)
            results.push(line);
        }
    }

    results
}