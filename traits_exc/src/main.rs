use std::fmt::{Debug, Display};

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

struct Pair<T> {
    x: T,
    y: T
}

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x, y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {

        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
        
    }
}

fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

fn som_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    23
}

fn main() {
    let news_article: NewsArticle = NewsArticle {
        headline: String::from("Massive elephant spotted"),
        location: String::from("Kolkata"),
        author: String::from("Saitama"),
        content: String::from("Saitama killed that elephant with one punch like nothing")
    };

    let tweet: Tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false
    };

    // println!("1. new headline: {}", news_article.summarize());
    println!("1. new tweet: {}", tweet.summarize());
    notify(tweet);
}
