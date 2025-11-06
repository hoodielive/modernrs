struct Textmatch(usize, String);

enum HttpResultCode
{
    Ok = 200,
    NotFound = 404,
    Teapot = 418
}

fn main() 
{
    let x= 42i32;
    println!("{}", x);

    let m = Textmatch(12, "needle".to_string());

    let code = HttpResultCode::NotFound;

    assert_eq!(m.0, 12);
    assert_eq!(code as i32, 404);
}
