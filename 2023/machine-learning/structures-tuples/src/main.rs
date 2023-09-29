#[derive(Debug)]
struct Planet {
    c02: f32,
    nitrogen: f32,
}

// Traits are meant to implement zero-cost abstractions for types.
// It refers to the idea that using higher-level abstractions 
// (like functions, objects, or patterns) shouldn't impose a runtime 
// overhead compared to writing the equivalent code "manually" at a 
// lower level. In simpler terms, with zero-cost abstractions, you 
// can write code that's more readable, modular, and maintainable 
// without sacrificing performance. Because structs and other data 
// structures do not have their own functionality. These functionalities
// are defined using traits. Traits are similar to interfaces in OOP.

trait Atmosphere {
    // c02 and nitrogen are members of the trait, whereas 
    // summarize is a method
    fn new(c02: f32, nitrogen: f32) -> Self;
    fn amount_of_other_gases(&self) -> f32;
    fn summarize(&self);
}

// To implement methods on structs use impl block.
// In this you have something that resembles objects,
// which means we have a 'type' that encapsulates data
// and behavior.

impl Atmosphere for Planet {
    fn new(c02: f32, nitrogen: f32) -> Planet {
        Planet { c02: c02, nitrogen: nitrogen }
    }

    fn amount_of_other_gases(&self) -> f32 {
        100.0 - self.c02 - self.nitrogen
    }

    fn summarize(&self) {
        let other_gases = self.amount_of_other_gases();
        println!("
            For planet {planet:?}: c02 = {c02},
            nitrogen={nitrogen}, 
            other_gases={other_gases}",
            planet=self, 
            c02=self.c02, 
            nitrogen=self.nitrogen, 
            other_gases=other_gases 
        );
    }
}

fn main() {
    let earth = Planet { c02: 0.04, nitrogen: 78.09 };
    println!("{:?}", earth);

    let mars = Planet { c02: 95.32, nitrogen: 2.7 };
    println!("{:?}", mars);

    earth.summarize();
    mars.summarize();
}
