#[derive(Debug, Eq, PartialEq)]
enum Access
{
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access>
{
    match name
    {
        "admin"     => Some(Access::Admin),
        "gary"      => Some(Access::User),
        _           => None,
    }
}

fn prophecy(name: &str) -> Option<Access>
{
    Some(Access)
}

fn root() -> Option<Access>
{
    Some(Access::Admin)
}

fn part_1() -> bool
{
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access>
{
    maybe_access("root").or_else(|| root())
}

fn part_3() -> Access 
{
    maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}
