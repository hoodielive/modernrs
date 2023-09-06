struct Emphasis {
    size: i8,
    user_size: i32,
}

#[derive(Debug)]
pub struct Magnify {
    _gender: String,
    _age: i32,
    _name: String,
}

impl Magnify {
    fn new(self) {
        self.gender;
        self.age;
        self.name;
    }
}

struct Alexer<T> {
    comedy: T,
}

fn main() {
    let tup: (u8, i8, f32) = (1, 2, 3.0);
    println!("this is a tuple {:?}.", tup);

    let knowledge = Emphasis {
        size: 32,
        user_size: 324,
    };

    println!(
        "Send the values {}, {}.",
        knowledge.size, knowledge.user_size
    );

    let reser = Alexer { comedy: "Love" };

    println!("What is this generic? {}.", reser.comedy);

    let impl_magnify = Magnify {
        _gender: ("Boy").to_string(),
        _age: 32,
        _name: "Ose".to_string(),
    };

    println!(
        "Here is an implementation of the Magnify struct {:?}: ",
        impl_magnify
    );
}
