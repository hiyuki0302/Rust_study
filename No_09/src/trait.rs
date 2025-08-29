pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}


impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 引数としてのトレイト
pub fn notify（item： &impl Summary）{ 
    println!("Breaking news! {}", item.summarize());
}

// 複数のトレイト境界を＋構文で指定。
pub fn notify_2 <T: Summary + Display>(item: &T) { // pub fn notify_2 (item: &(impl Summary + Diplay)) {}
}

// where句を使ったトレイト境界
pub some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug

fn main(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize_author());
    
    notify(&article) // 引数としてのトレイト
}