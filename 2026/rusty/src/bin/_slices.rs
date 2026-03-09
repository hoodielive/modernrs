



fn main()
{
   // Slices are references to a contiguous sequence of elements in a collection.

    let tweet = String::from(
        "This is my tweet and it's very very long"
    );
    
    let trimmed_tweet: &str = trim_tweet(&tweet);
    println!("{}", trimmed_tweet);

    // What would happen if our string was a string literal

    let tweet2 = "This is my tweet and it is very very long";
    let trimmed_tweet2 = trim_tweet(tweet2);

    println!("{trimmed_tweet2}");
   
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);
    // String { growable, heap allocated string (UTF-8 encoded)}
    // str { immuatable sequence of UTF-8 bytes somewhere in memory (stack, heap, or stack memory)}
    // Handle behind a reference &str because length of sequence is unknown at compile time.
}

// reference to a String because we don't want to take ownership of the String
fn trim_tweet(tweet: &str) -> &str // return a string slice.
{
    &tweet[..20]   
}
