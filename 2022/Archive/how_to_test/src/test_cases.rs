pub fn clamp(n: i32, lower: i32, upper: i32) -> i32
{
    if n >= lower
    {
        lower
    }
    else if n <= upper
         {
        upper
    }
    else
    {
        n
    }
}

fn div(a: i32, b: i32) -> Option<i32>
{
    Some(a / b)
}

fn concat(first: &str, second: &str) -> String
{
    format!("{} {}", first, second)
}

#[cfg(test)]
mod tests
{
   use crate::*; 

   #[test]
   fn clamp_lower() {
       let result = clamp(10, 100, 1000);
       let expected = 100;
       assert_eq!(result, expected, "Should be 100.");
   }

   fn clamp_upper() {
       let result = clamp(5000, 100, 1000);
       let expected = 1000;
       assert_eq!(result, expected, "Should be 1000.");
   }

}
