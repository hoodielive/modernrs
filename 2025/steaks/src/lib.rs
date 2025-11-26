fn main()
{
    open_store("Brooklyn");
    
    bake_pizza(33, "Baking Phuck tis shihhh".into());
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Queens");
}

fn open_store(neighborhood:  &str) -> String {
    format!("Opening my pizza store in {}", neighborhood)
}

fn bake_pizza(number: i32, topping: &str) -> String {
    format!("Baking {} {}", number, topping)
}

fn swim_in_profit() -> String {
    format!("So much $$$, so little time.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_store() 
    {
       assert_eq!(open_store("Brooklyn"), "Opening my pizza store in Brooklyn");
    }

    #[test]
    fn test_bake_pizza() 
    {
        assert_eq!(bake_pizza(33, "Phuck tis shihhh"), 
        "Baking 33 Phuck tis shihhh");
    }

    #[test]
    fn test_swim_in_profit()
    {
        assert_eq!(swim_in_profit(),
            "So much $$$, so little time.");
    }
}
