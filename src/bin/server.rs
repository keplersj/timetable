use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;
use timetable::web::handlers;

fn main() -> std::io::Result<()> {
    let sys = actix_rt::System::builder().stop_on_panic(false).build();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let address = "127.0.0.1:8088";
    let create_app = || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(handlers::index))
            .route(
                "/schedule",
                web::post().to(handlers::create_scheduled_request),
            )
            .route(
                "/schedule/{id}",
                web::delete().to(handlers::delete_scheduled_request),
            )
            .route(
                "/schedule/{id}",
                web::get().to(handlers::get_scheduled_request_status),
            )
            .route(
                "/schedule/{id}",
                web::put().to(handlers::modify_scheduled_request),
            )
    };

    HttpServer::new(create_app).bind(address)?.start();

    println!("Started http server: {}", address);
    sys.run()
}
