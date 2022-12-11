#![allow(dead_code)]
#![allow(unused_variables)]


pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary
{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String
    {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait SummaryAll
{
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle
{
    fn summarize(&self) -> String
    {
        format!
        (
            "{}, by {} ({}).", 
            self.headline, 
            self.author, 
            self.location,
        )
    }
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet
{
    fn summarize(&self) -> String
    {
        format!
        (
            "{}: {}",
            self.username,
            self.content
        )
    }
}

pub struct Prophecy
{
    threemonthfromnow: String,
}


impl SummaryAll for Prophecy
{
   fn summarize(&self) -> String
   {
       format!("{} ", self.threemonthfromnow)
   }
}

fn main()
{
    let tweet = Tweet
    {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let prophecy01 = Prophecy
    {
        threemonthfromnow: String::from("You are about to be wealthy."),
    };

    println!("The future says: {}", prophecy01.summarize());
}
