#![allow(dead_code)]
#![allow(unused_variables)]

struct Point<T, U>
{
    x: T,
    y: U,
}

trait Overview
{
    fn overview(&self) -> String;
}

struct Course
{
    headline: String,
    author: String,
}

struct AnotherCourse
{
    headline: String,
    author: String,
}

impl Overview for Course
{
    fn overview(&self) -> String
    {
        format!("{}, {}", self.author, self.headline)
    }
}


impl Overview for AnotherCourse
{
    fn overview(&self) -> String
    {
        format!("{} {}", self.author, self.headline)
    }
}

fn main() 
{
    let coord_01 = Point { x: 5.0, y: 5.0 };
    let coord_02 = Point { x: 'x', y: 9 };

    let course_01 = Course 
    { 
        headline: String::from("Every Which Way But Loose"), 
        author: String::from("Clint Eastwood")
    };

    let course_02 = AnotherCourse 
    { 
        headline: String::from("Sanford and Son"), 
        author: String::from("Red Foxx"),
    };

    println!("{}",course_01.overview());
    println!("{}",course_02.overview());

    call_overview(&course_01);
    call_overview_simplified(&course_02);
}

fn call_overview(item: &impl Overview)
{
    println!("Overview: {}", item.overview())
}

fn call_overview_simplified<T: Overview>(item: &T)
{
    println!("Overview: {}", item.overview())
}

fn overview_02(item1: &impl Overview, item2: &impl Overview) {};
fn overview_03<T: Overview>(item1: &T, item2: &T) {};
fn overview_04(item1: &impl Overview + AnotherTrait) {};
fn overview_05<T: Overview + AnotherTrait>(item1: &T, item2: &T) {};
