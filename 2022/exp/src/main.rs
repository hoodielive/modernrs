#![allow(unused_variables)]
#![allow(dead_code)]

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

   let _fruit32: Fruit<u32> = Fruit
   {
       apples: 5_u32,
       bananas: 10_u32,
   };
   
}
