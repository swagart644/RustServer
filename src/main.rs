use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("rust_server"),
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(appname)
            .service(hello)
            .service(echo)
            .service(count)
            .route("/getnumb", web::get().to(get_number))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_number() -> impl Responder {
    let number: i32 = 123;
    HttpResponse::Ok().body(number.to_string())
}

#[get("/counter")]
async fn count(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[get("/appname")]
async fn appname(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; 
    format!("Hello {app_name}!")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}