use actix_web::HttpResponse;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("login.html"))
}
