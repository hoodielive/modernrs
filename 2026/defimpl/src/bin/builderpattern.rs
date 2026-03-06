struct BurgerBuilder
{
    
}

struct Burger {
    patty_count: i32,
    vegetarian: bool, 
    cheese: bool, 
    bacon: bool, 
    salad: bool,
}

impl Burger {
    fn print(&self)
    {
       let pretty_patties = if self.patty_count == 1 {
           "patty"
       }  else {
           "patties"
       };
       let pretty_bool = |val| if val {""} else {"no"};
       let pretty_vegetarian = if self.vegetarian { "vegetarian" } else { "" };
       println!(
           "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad", pretty_vegetarian, self.patty_count, pretty_patties, pretty_bool(self.cheese), pretty_bool(self.bacon), pretty_bool(self.salad)
       )
    }
}

struct BurgerBuilder {
    patty_count: i32,
    vegetarian: bool, 
    cheese: bool, 
    bacon: bool,
    salad: bool,
}

impl BurgerBuilder {
    fn new() -> Self
    {
        BurgerBuilder {
            patty_count: 1, 
            vegetarian: false,
            cheese: false,
            bacon: false, 
            salad: true,
        }
    }
}

fn main()
{
    let normal_burger = BurgerBuilder::new().build();
    
    let cheese_burger = BurgerBuilder::new()
        .cheese(true)
        .salad(false)
        .build();
    
    let veggie_bigmac = BurgerBuilder::new()
        .vegetarian(true)
        .patty_count(2)
        .build();
    

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }

    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }

    if let Ok(veggie_bigmac) = veggie_bigmac {
        veggie_bigmac.print();
    }

    let invalid_burger = BurgerBuilder::new()
        .vegetarian(true)
        .bacon(true)
        .build();

    if let Err(error) = invalid_burger {
        println!("Failed to print burger: {}", error)
    }

    let cheese_burger = cheese_burger_builder.build();
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("Cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }
}
