#![allow(dead_code)]
#![allow(unused_variables)]

pub fn demo()
{
    let a: Option<i32> = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
   
    // filter borrow's the number.
    let a_filtered = a.filter(|num| num == &1);
    let a_or_else = a.or_else(|| Some(5));
//    let unwrapped = a.unwrap_or(|| 0);
}
