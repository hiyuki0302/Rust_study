〇スカラー型
    ・整数型
        少数なし。符号付きが「i」、符号なしが「u」
        iは-128~127,uは0~255
        違いはメモリ効率、表現できる範囲、型の安全性（配列のインデックスは負の値にならない。）
        基準はi32型。最速らしい。
    ・浮動小数点
        f32,f64型で32ビットと64ビットの違い。
        基準はf64型。
    ・論理値
        bool型。TrueかFalse。
    ・文字
        Char型。ユニコードのスカラー値を表せる。
        日本語や絵文字、スペースなにからなにまでCharにお任せ。

〇複合型
    ・タプル型
        丸括弧。タプル内の値はそれぞれが別の型でもいい。let tup: (i32, f64, u8) = (500, 6.4, 1);
        取り出し方→tup.0, tup.1, tup.2
    ・配列型
        角括弧。配列全体の要素がすべて同じ型でないといけない。
        let a: [i32; 5] = [1, 2, 3, 4, 5];　型と要素の個数を指定
        let a = [3; 5];　→　let a = [3, 3, 3, 3, 3];　になるよ。初期化といわれてるらしいよ💀
        他の言語と異なり、サイズを変えられない！！固定長！ベクタ長というものは伸縮自在・・・♠
        1年の月の名前を扱うときとか固定長がいいかもね
        取り出し方→a[0], a[1]....
    
〇関数
    ・各仮引数の型を宣言しなければならない。
        fn print_labeled_measurement(value: i32, unit_label: char)
    ・関数本体は文と式を含む
        fn main() {
            let y = {
                let x = 3; →文（セミコロンがついている。） 
                x + 1 →式
            };
            println!("The value of y is: {}", y);
        }

    ・戻り値のある関数
        returnで早期から値を返すこともできるが、暗黙的に最後の式を返してくれるよ。
        戻り値に名前は付けないが型宣言が必要なんだ・・・♠ 「->」これで。

        fn main() {
            let x = plus_one(5);

            println!("The value of x is: {}", x);
        }

        fn plus_one(x: i32) -> i32 {
            x + 1;
        }

        上の式はエラーになる。i32型を返すといってるのに、文を返しているため。
        返り値はなしで、main()実行時、let x = ()になってしまう。（空のタプル）

〇制御フロー
    ・if文
        let文の中でifを使うと以下のようになる。
            let condition = true;
            let number = if condition { 5 } else { 6 };
        同じ変数には同じ型のものしか代入しようとしてはいけない。コンパイル時に迷うからね...♠
            let condition = true;
            let number = if condition { 5 } else { "バンジーガム" };　数値型とStr型が混同している。
    
    ・ループ
        fn main() {
            let mut count = 0;
            'counting_up: loop { 'counting_upというラベル
                println!("count = {}", count);
                let mut remaining = 10;

                loop {
                    println!("remaining = {}", remaining);　
                    if remaining == 9 { 
                        break; 内側のループブレイク
                    }
                    if count == 2 {
                        break 'counting_up; 外側のループブレイク
                    }
                    remaining -= 1;
                }

                count += 1;
            }
            println!("End count = {}", count);
        }
    ・forループ
        fn main() {
            let a = [10, 20, 30, 40, 50];

            for element in a {
                println!("the value is: {}", element);
            }
        }

        fn main() {
            for number in (1..4).rev() {
                println!("{}!", number);
            }
            println!("LIFTOFF!!!");
        }

        pythonとほぼ同じ...♠

----------------------------------------------------------------------------------

〇所有権
    ☆ルール☆
        ・Rustの各値は、所有者と呼ばれる変数と対応している。
        ・いかなる時も所有者は一つである。
        ・所有者がスコープから外れたら、値は破棄される。
        ※スコープ・・・要素が有効になるプログラム内範囲。{}の中。
    
    ・メモリの確保と解放
        ・ガベレージ付きの言語だと、自動でメモリを解放してくれる。
        ・が、GCがないのであればメモリをOSに返還しなければならない。
        →Drop関数
    
    