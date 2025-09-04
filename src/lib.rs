use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

// This is the request handler function that returns something that implements the responder trait (can be
// converted to http response)
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}

// FIX CI pipeline before pushing
