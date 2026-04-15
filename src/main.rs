use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8081");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}