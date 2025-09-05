use actix_web::HttpResponse;

// This is the request handler function that returns something that implements the responder trait (can be
// converted to http response) (this could also be impl Responder no difference)
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
