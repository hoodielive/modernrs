use actix_web::{Responder, post};

#[post("/me")]
pub async fn get_profile() -> impl Responder {
    "Profile"
}

#[post("/me")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}
