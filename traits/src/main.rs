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


// returning types the implement traits
fn returns_summarizable() -> impl Summary {
    Tweet { 
        username: String::from("horse_eboks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
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

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}

use std::fmt::Debug;
use std::fmt::Display;

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


// Conditionally implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T ) -> Self {
        Self { x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        }else {
            println!("The largest member is y = {}", self.y);
        }
    }
}