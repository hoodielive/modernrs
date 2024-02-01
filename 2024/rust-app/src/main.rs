#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str
{
    "Howdy, I am saying hello for now()"
}

#[rocket::main]
async fn main()
{
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
