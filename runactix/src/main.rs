use actix_web::{HttpServer, App};
use std::sync::{Arc, Mutex};

struct Messenger
{
    message: String,
}

struct MutableState
{
    messenger: Mutex<Messenger>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let app_data = actix_web::web::Data::new(MutableState
        {
            messenger: Mutex::new(Messenger { message: "hello".to_string() })
        });
    
    HttpServer::new(move || 
    {
        App::new()
            .app_data(app_data.clone())
            // .route()
    })
    .bind("127.0.0.1", 8001)?
    .run()
    .await
}


