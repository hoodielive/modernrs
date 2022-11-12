pub struct Greeting {
    name: String
}


impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self 
    {
        name.as_ref().to_string();
    }

}
