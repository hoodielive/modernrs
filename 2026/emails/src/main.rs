use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn alarm(req: HttpRequest) -> impl Responder {
    let prophecy = req.match_info().get("").unwrap_or("the end");
    format!(
        "You guys really have to start listening to the truth {}.",
        &prophecy
    )
}

async fn parsley(req: )

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/{prophecy}", web::get().to(alarm))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
        voltaory
}
