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
    
    ・let s1 = String::from("hello");　→　可変サイズのため、ヒープメモリに保存される。
    　let s2 = s1;
    　上記のような文はムーブと呼ばれ、自動でs1が無効化される。
    　データポインタが2つとも同じ場所を指しており、スコープを抜けた場合同じ場所のヒープメモリを解放しようとしてしまう。（二重解放エラー）
    　整数のような決まっているサイズを持つ型はスタック上に保持されるのでムーブされない。
        fn main() {
            let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                                // ムーブする

            let s2 = String::from("hello");     // s2がスコープに入る

            let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                                // 戻り値もs3にムーブされる
        } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
        // 何も起きない。s1もスコープを抜け、ドロップされる。

        fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                                    // 呼び出した関数にムーブする

            let some_string = String::from("hello"); // some_stringがスコープに入る

            some_string                              // some_stringが返され、呼び出し元関数に
                                                    // ムーブされる
        }

        // takes_and_gives_backは、Stringを一つ受け取り、返す。
        fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

            a_string  // a_stringが返され、呼び出し元関数にムーブされる
        }

    　値を返すことでも所有権は移動する。あー、もうめちゃくちゃだよ

----------------------------------------------------------------------------------
〇参照と借用
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        // '{}'の長さは、{}です
        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    ・&をつけると参照になる。また、関数の引数に参照した値を用いることを借用と呼ぶ。デフォルトでは、借りたものはそのまま返さないといけないので不変（値を変えられない）
    　また。所有権の移動が発生しないのでスコープを抜けてドロップしても参照された値は残る。
    　重要：Stringと$Stringは型が別のものとして認識される。例えば上記のコードだと、let len = calculate_length(s1)にしてしまうと、$Stingを引数にしているためコンパイルエラーになる。

    ・可変な参照
    mutをつければOK。しかし、ルールがある。
    ✅ 複数の不変参照
    ✅ 1つの可変参照
    同一のスコープ内ではどちらかのみが可能になる。
    let s1 = String::from("hello");
    let r1 = &s1; 
    let r2 = &s1;
    let r3 = &mut s1;　

    また、以下のコードのように参照元の値が解放されてしまうとエラーになる。
    fn dangle() -> &String { // dangleはStringへの参照を返す

        let s = String::from("hello"); // sは新しいString

        &s // String sへの参照を返す
    } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。

〇スライス
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    .iter()で各要素を参照(&u8)としてイテレートし、enumerate()でインデックス付きのタプル(usize, &u8)を生成。
    enumerate()の各要素は(i, ref)の形で、iはインデックス(usize)、refは&u8。
    パターンマッチングと言われ、参照型をマッチさせて自動的に参照を外して(デリファレンス)item変数に中身の値(u8)をバインドしてくれる。これにより参照値ではなく直として扱える。
    &itemにしないと、どこかで別にデリファレンスしないといけないよ☆
    

