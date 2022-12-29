use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState
{
    app_name: String,
}

async fn index3() -> impl Responder
{
    "Oyeku says Hello!"
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String
{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

async fn index2(data: web::Data<AppState>) -> impl String
{
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder
{
    HttpResponse::Ok().body("Howdy there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::Data::new(
                    AppState {
                        app_name: String::from("Oyeku Meji is in the building!"),
                    }
                )
            )
            .service(index)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/index.html", web::get().to(index3))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}