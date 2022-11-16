fn main() 
{
    let mut s: String = String::from("Hello World");
    let hello: &str = &s[..5];
    let world: &str = &s[..];
    let word: &str = first_word(&s);
}

fn first_word(s: &String) -> &str
{
    let bytes: &[u8] = s.as_bytes();

    for (i: &str, &item: u8) in bytes.iter().enumerate() 
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    &s[..]
}
