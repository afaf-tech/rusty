// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
pub struct NewsArticle { 
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String{
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String{
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String{
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String{
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary){
//     println!("Breaking news! {}", item.summarize());
// }

// this generic is limited to something that implements the summary trait
pub fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet{
        username: String::from("@johndoe"),
        content: String::from("Hello world!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary : {}", tweet.summarize());
    println!("Article summary : {}", article.summarize());

    notify(&article)
}
pub trait Display{}

use std::fmt::Debug;

// JUST SOME EXAMPLE OF GENERIC USAGE IN TRAIT
pub fn notify2_param(item1: &(impl Summary + Display), item2: &impl Summary){
    //...
}

pub fn some_function<T: Display + Clone, U: Clone + Debug > (t: &T, u: &U) -> i32 {
    //...
    return 1;
}

pub fn some_function2<T, U> (t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U : Clone + Debug
{
    // ...
    return 1;
}