pub fn extend_message(mut a: String) -> String // which changes lifetime but does not implement the
                                               // Copy trait
{
    a.push_str(" World")
}
