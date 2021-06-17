use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// common
struct BidRequest {
    body: String,
}
struct BidResponse {
    body: String,
}

struct SspZebra {}

trait SspApi<HttpReq, HttpRes> {
    fn parse(body: HttpReq) -> BidRequest;
    fn format(res: BidResponse) -> HttpRes;
}

impl SspApi<String, String> for SspZebra {
    fn parse(body: String) -> BidRequest {
        BidRequest { body }
    }
    fn format(res: BidResponse) -> String {
        res.body
    }
}

struct SspController {}

impl SspController {
    fn process(req: BidRequest) -> BidResponse {
        BidResponse { body: req.body }
    }
}

#[post("/rtb/zebra")]
async fn rtb_zebra(req_body: String) -> impl Responder {
    let req = SspZebra::parse(req_body);
    let res = SspController::process(req);
    let body = SspZebra::format(res);
    HttpResponse::Ok().body(body)
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
