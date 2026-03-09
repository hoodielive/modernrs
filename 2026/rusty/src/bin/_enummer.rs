#![allow(dead_code)]

#[derive(Debug)]
struct Product
{
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool
}

impl Product
{
    fn new(name: String, category: ProductCategory, price: f32, in_stock: bool) -> Self
    {
        Self {
            name, 
            category,
            price,
            in_stock
        }
    }
}

#[derive(Debug)]
enum ProductCategory
{
    Books,
    Clothing,
    Electronics,
}

fn main()
{
    let category = ProductCategory::Electronics;

    let product = Product::new(
        String::from("Larry"),
        category,
        2334.43,
        true
    );

    println!("{:#?}", product);
}
