use actix_web::{get, web, HttpResponse, Responder};
use reqwest;
use serde_json::Value;

#[get("/horoscope/{sign}")]
async fn horoscope(sign: web::Path<String>) -> impl Responder {
    let url = format!("https://astrology-api.example.com/horoscope?sign={}", sign);

    let response = reqwest::get(&url).await;

    match response {
        Ok(resp) => {
            let horoscope: Result<Value, _> = resp.json().await;
            match horoscope {
                Ok(horoscope_data) => HttpResponse::Ok().json(horoscope_data),
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to parse JSON: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch from API: {}", e)),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(horoscope);
}

