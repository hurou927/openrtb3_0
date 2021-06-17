use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/rtb/zebra")]
async fn rtb_zebra(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn metrics() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let host = format!("127.0.0.1:{}", port);

    println!("start: {}", host);

    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(rtb_zebra)
            .route("/metrics", web::get().to(metrics))
    })
    .bind(host)?
    .run()
    .await
}
