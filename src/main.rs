use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::env;
use std::sync::Mutex;


struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads.
}


async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let app_name: &String = &data.app_name;

    // Get the counter's MutexGuard and access the counter:
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    // Respond with the app name and the counter value:
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("Hey there, <b>{app_name}</b>!<br/>Request number: {counter}."))
}

async fn manual_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

async fn manual_counter_add(data: web::Data<AppStateWithCounter>, req: HttpRequest) -> String {
    let number_parse = req.match_info().query("number").parse::<i32>();
    match number_parse {
        Ok(number) => {
            let mut counter = data.counter.lock().unwrap();
            *counter += number;

            format!("Added: {number}\nRequest number: {counter}")
        }
        Err(e) => {
            format!("Invalid number! ('{e}')")
        }
    }
}

async fn manual_cat() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/cat.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let host = env::var("HOST").expect("Host not set.");
    let port = env::var("PORT").expect("Port not set.");

    // Note: web::Data created _outside_ HttpServer::new closure.
    let counter = web::Data::new(AppStateWithCounter {
        app_name: String::from("rust-actix-web"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // Move counter into the closure:
        App::new()
            .app_data(counter.clone()) // <- Register the created data.
            .route("/", web::get().to(index))
            .route("/counter", web::get().to(manual_counter))
            .route("/counter/add/{number}", web::get().to(manual_counter_add))
            .route("/cat", web::get().to(manual_cat))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}