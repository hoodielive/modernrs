#![allow(unused_variables)]
#![allow(dead_code)]

struct Person<Name, Age>
{
    name: Name,
    age: Age,
}

impl Person<String, u32>
{
    fn new(self) -> Person<String, u32>
    {
        Person {
            name: "Default".to_owned(),
            age: 0,
        }
    }

    fn greet(person: &Person<String, f64>)
    {
        println!("Hello, {}", person.name);
    }
}

struct Fruit
{
    apples: u32,
    bananas: u32,
}

struct FruitString
{
    apples: String,
    bananas: String,
}

struct Fruity<T>
{
    apples: T,
    bananas: T,
}

impl Fruit
{
fn price(self) -> u32
    {
        self.apples * 8 + self.bananas * 12
    }

    fn new_fruit() -> Fruit
    {
        Fruit 
        {
            apples: 10,
            bananas: 5,
        }
    }
}

fn main() 
{
   let fruit = Fruit::new_fruit();
   let price = fruit.price();
   println!("Price is {}", price);

   let x = 5;

   let fruitu32: Fruity<u32> = Fruity
   {
       apples: 5_u32,
       bananas: 10_u32,
   };

   let fruitu64: Fruity<f64> = Fruity
   {
       apples: 6.4_f64,
       bananas: 8.3_f64,
   };

   let fruitstring: Fruity<String> = Fruity
   {
       apples: "apples".to_owned(),
       bananas: "bananas".to_owned(),
   };

   let osa: Person<String, u32> = Person
   {
       name: "Osa Oso".to_owned(),
       age: 900000_u32,
   };
   
   println!("Could you tell me a little more {}. And is your number {}?", osa.name, osa.age);
    
   let person: Person<String, u32> = new Person<String, u32>();
   
}
