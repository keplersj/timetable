use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://github.com/keplersj/timetable")
        .finish()
}

fn main() {
    let address = "127.0.0.1:8088";
    let create_app = || App::new().route("/", web::get().to(index));

    HttpServer::new(create_app)
        .bind(address)
        .unwrap()
        .run()
        .unwrap();
}
