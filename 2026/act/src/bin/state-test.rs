use actix_web::{ web, get, App, HttpServer };

struct AppState
{
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String
{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
       App::new()
           .app_data(web::Data::new(AppState {
               app_name: String::from("Actix Web Shihhh"),
           }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
