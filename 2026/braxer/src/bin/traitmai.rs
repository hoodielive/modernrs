use std::cmp::{ Eq, Ordering, PartialEq };

trait Summary 
{
    fn summarize(&self) -> String
    {
        format!("Not Implemented!")
    }
}

struct FacebookPost
{
    author: String,
    content: String,
}

struct InstagramPost
{
    author: String,
    description: String,
}

impl Summary for FacebookPost
{
    fn summarize(&self) -> String
    {
        format!("{}: {}", self.author, self.content)
    }
}

impl Summary for InstagramPost
{
    fn summarize(&self) -> String
    {
        format!("{}: {}", self.author, self.description)
    }
}

impl PartialEq for FacebookPost
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.author == other.author && self.content == other.content
    }
}

impl PartialEq for InstagramPost
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.author == other.author && self.description == other.description
    }
}

impl Eq for FacebookPost {}
impl Eq for InstagramPost {}

fn main()
{
   let fb_post = FacebookPost {
      author: String::from("Joji"),
      content: String::from("Blablabla"),
   };

   println!("{}", fb_post.summarize());

   let ig_post = InstagramPost {
       author: String::from("Jeffrey"),
       description: String::from("Dont eff with me son!"),
   };

   println!("{}", ig_post.summarize());
}
