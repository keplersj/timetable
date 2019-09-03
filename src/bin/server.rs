use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://github.com/keplersj/timetable")
        .finish()
}

fn create_scheduled_request() -> impl Responder {
    HttpResponse::NotImplemented()
}

fn delete_scheduled_request() -> impl Responder {
    HttpResponse::NotImplemented()
}

fn get_scheduled_request_status() -> impl Responder {
    HttpResponse::NotImplemented()
}

fn modify_scheduled_request() -> impl Responder {
    HttpResponse::NotImplemented()
}

fn main() {
    let address = "127.0.0.1:8088";
    let create_app = || {
        App::new()
            .route("/", web::get().to(index))
            .route("/schedule", web::post().to(create_scheduled_request))
            .route("/schedule/{id}", web::delete().to(delete_scheduled_request))
            .route(
                "/schedule/{id}",
                web::get().to(get_scheduled_request_status),
            )
            .route("/schedule/{id}", web::put().to(modify_scheduled_request))
    };

    HttpServer::new(create_app)
        .bind(address)
        .unwrap()
        .run()
        .unwrap();
}
