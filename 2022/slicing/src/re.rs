#![allow(unused_variables)]
#![allow(dead_code)]

pub fn reimplement(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }
    s.len()
}

pub fn just_return_slices(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    &s[..]
}
