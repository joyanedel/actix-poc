use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod api;

struct DummyData {
    peo_quality: u8,
}

struct DummyDataWithMutex {
    counter: Mutex<u8>,
}

#[get("/hello")]
async fn get_hello_world(data: web::Data<DummyData>) -> impl Responder {
    let payload = &data.peo_quality;
    HttpResponse::Ok().body(format!("Hola Jesús, tu calidad de peo es {}/255", payload))
}

#[get("/mutex")]
async fn mut_counter(data: web::Data<DummyDataWithMutex>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Hola Jesús, contador: {}", counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(DummyDataWithMutex {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // import routes defined in other modules
            .configure(api::config)
            // Insert shared state
            // Simple shared data
            .app_data(web::Data::new(DummyData { peo_quality: 5 }))
            // Simple mutex shared data
            .app_data(counter.clone())
            // scope
            .service(web::scope("/v1").service(get_hello_world))
            // mutex values
            .service(mut_counter)
            // guards. If host is not google.com then the server wont respond
            .service(web::scope("/v2").guard(guard::Host("google.com")).route(
                "/",
                web::to(|| async { HttpResponse::Ok().body("Le puso") }),
            ))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
