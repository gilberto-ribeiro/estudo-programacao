use aggregator::{self, NewsArticle, Pair, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // println!("{}", article.summarize());
    // println!("{}", tweet.summarize());
    aggregator::notify(&article);
    aggregator::notify(&tweet);

    let pair = Pair::new(Some(42).unwrap(), Some(15).unwrap());
    pair.cmp_display()
}
