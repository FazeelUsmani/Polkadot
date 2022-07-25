mod basic_trait;
use basic_trait::*;


fn main() {

    println!("Hello, world!");

    let tweet = Tweet {
        username: "Fazeel".to_string(),
        content: String::from("I'm back"),
        reply: false,
        retweet: true,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
