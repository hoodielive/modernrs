#![allow(dead_code)]
#![allow(unused_variables)]


pub fn print_type_of<T>(_: &T)
{
    println!("{}", std::any::type_name::<T>())
}

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
    fn summarize_author(&self) -> String
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
    fn summarize_author(&self) -> String
    {
        format!("@{}", self.username)
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

pub fn notify<T: Summary>(item: &T)
{
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary
{
    Tweet
    {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know."),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;
struct Pair<T>
{
    x: T,
    y: T,
}

impl<T> Pair<T>
{
    fn new(x: T, y: T) -> Self
    {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T>
{
    fn cmp_display(&self)
    {
        if self.x >= self.y
        {
            println!("The Largest member is x = {}", self.x)
        }
        else
        {
            println!("The Largest member is y = {}", self.y)
        }
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

    let s = 3.to_string();

    println!("Printed s = {}", &s);
    print_type_of(&s);
}
