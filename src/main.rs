mod client_for_csv_to_json;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_top() -> impl Responder {
    HttpResponse::Ok().body("top page")
}

#[get("/csv")]
async fn get_json() -> impl Responder {
    let result = client_for_csv_to_json::module_csv_to_json::get_csv_to_json().await;
    HttpResponse::Ok()
    .content_type("application/json")
    .body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_top)
            .service(get_json)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}