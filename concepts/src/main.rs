use std::collections::HashMap;
use std::sync::LazyLock;

static HASHMAP: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| 
{
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
}); 

dbg!(&*HASHMAP);
fn main() 
{
        
}
