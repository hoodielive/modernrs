struct Node
{
    elem: i32,
    next: Option<Box<Self>>,
}

struct Foo(i32);

impl Foo(i32)
{
    fn new()->Self
    {
        Self(0)
    }
}

assert_eq!(Foo::new().0, Foo(0).0);

struct Wrap<T>
{
    elem: T,
}

impl<T> Wrap<T>
{
    fn new(elem: T)->Self
    {
        Self { elem }
    }
}


trait Example
{
    fn example()->Self;
}


impl Example for Foo
{
    fn example()->Self
    {
        Self(42)
    }
}

assert_eq!(Foo::example().0, Foo(42).0);
