#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[get("/")]
fn hello() -> &'static str
{
    "Howdy from the index."
}

#[rocket::main]
async fn main()
{
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
    
}
