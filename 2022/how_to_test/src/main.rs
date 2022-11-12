#![allow(dead_code)]

mod test_cases;

fn all_caps(word: &str) -> String
{
    word.to_uppercase()
}

fn main()
{
    test_cases::tests();

}

#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn check_all_caps()
    {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "String should be all uppercase.");
    }
}
