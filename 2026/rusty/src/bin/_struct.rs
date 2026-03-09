#![allow(dead_code)]
struct Product
{
    name: String,
    price: f32,
    inStock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            inStock: true,
        }
    }
    
    fn get_default_sales_tax() -> f32 {
        0.1
    }
    
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
       let name = self.name; 
       println!("{name} was bought!");
       123
    }
}

fn main()
{
   let mut book = Product::new(
       String::from("Boo"),
       30.0
   );
   
   let price = book.price;
   book.inStock = false;

   let sales_tax = book.calculate_sales_tax();
   println!("Sales tax: {}", sales_tax);
   
   book.set_price(1.0);
   book.buy();

}

