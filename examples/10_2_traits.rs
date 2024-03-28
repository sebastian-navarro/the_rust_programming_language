use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // Default output
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


// No verbose
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }


// Verbose mode
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple traits 
/*
pub fn notify<T: Summary + Display>(item: &T) {
*/

// Diferent variables with multiples traits
/*
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
*/

/* using where to do it more readable */
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    todo!();
}

fn main(){
    let tweet = Tweet {
        username: String::from("snavarro"),
        content: String::from("Hello Trait!"),
        reply: false,
        retweet: false
    };
    println!(" (1) new Tweet from {}", tweet.summarize());
}