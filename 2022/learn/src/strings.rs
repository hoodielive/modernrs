pub struct Greeting {
    name: String
}


impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Greeting: name.as_ref().to_string(),
    }
}
