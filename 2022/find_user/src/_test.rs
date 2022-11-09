#[cfg(test)]
mod test 
{
    use crate::*;

    #[test]
    fn check_part_1()
    {
        assert_eq!(part_1(), true, "Admins have an access level.");
    }

    #[test]
    fn check_part_2()
    {
        assert_eq!(part2(), Some(Access::Admin), "Root users have Admin access.");
    }

    #[test]
    fn check_part_3()
    {
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}
