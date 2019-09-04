use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use timetable::web::data;

fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://github.com/keplersj/timetable")
        .finish()
}

fn create_scheduled_request(payload: web::Json<data::ScheduleExecutionPayload>) -> impl Responder {
    println!("Create Scheduled Execution Payload: {:#?}", payload);
    HttpResponse::NotImplemented()
}

fn delete_scheduled_request(path: web::Path<(String)>) -> impl Responder {
    let id = path.into_inner();
    println!("Delete Scheduled Execution {}", id);
    HttpResponse::NotImplemented()
}

fn get_scheduled_request_status(path: web::Path<(String)>) -> impl Responder {
    let id = path.into_inner();
    println!("Get Scheduled Execution {}", id);
    HttpResponse::NotImplemented()
}

fn modify_scheduled_request(
    path: web::Path<(String)>,
    payload: web::Json<data::ScheduleExecutionPayload>,
) -> impl Responder {
    let id = path.into_inner();
    println!("Modify Scheduled Execution {} Payload: {:#?}", id, payload);
    HttpResponse::NotImplemented()
}

fn main() -> std::io::Result<()> {
    let sys = actix_rt::System::builder().stop_on_panic(false).build();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let address = "127.0.0.1:8088";
    let create_app = || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/schedule", web::post().to(create_scheduled_request))
            .route("/schedule/{id}", web::delete().to(delete_scheduled_request))
            .route(
                "/schedule/{id}",
                web::get().to(get_scheduled_request_status),
            )
            .route("/schedule/{id}", web::put().to(modify_scheduled_request))
    };

    HttpServer::new(create_app).bind(address)?.start();

    println!("Started http server: {}", address);
    sys.run()
}
