use actix_web::{web, HttpResponse,Responder};
use super::data;
use url::Url;
use chrono::Utc;

pub fn index() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "https://github.com/keplersj/timetable")
        .finish()
}

pub fn create_scheduled_request(payload: web::Json<data::ScheduleExecutionPayload>) -> impl Responder {
    println!("Create Scheduled Execution Payload: {:#?}", payload);
    HttpResponse::NotImplemented().json(data::ScheduledExecutionStatus {
        id: "42".to_string(),
        scheduled_timestamp: payload.0.scheduled_timestamp,
        target_webhook: payload.0.target_webhook,
        response_webhook: payload.0.response_webhook,
        status: data::ExecutionStatus::Waiting,
    })
}

pub fn delete_scheduled_request(path: web::Path<(String)>) -> impl Responder {
    let id = path.into_inner();
    println!("Delete Scheduled Execution {}", id);
    HttpResponse::NotImplemented()
}

pub fn get_scheduled_request_status(path: web::Path<(String)>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::NotImplemented().json(data::ScheduledExecutionStatus {
        id,
        scheduled_timestamp: Utc::now(),
        target_webhook: Url::parse("https://example.dev/hook").unwrap(),
        response_webhook: Url::parse("https://example.dev/hook").unwrap(),
        status: data::ExecutionStatus::Waiting,
    })
}

pub fn modify_scheduled_request(
    path: web::Path<(String)>,
    payload: web::Json<data::ScheduleExecutionPayload>,
) -> impl Responder {
    let id = path.into_inner();
    println!("Modify Scheduled Execution {} Payload: {:#?}", id, payload);
    HttpResponse::NotImplemented().json(data::ScheduledExecutionStatus {
        id,
        scheduled_timestamp: payload.0.scheduled_timestamp,
        target_webhook: payload.0.target_webhook,
        response_webhook: payload.0.response_webhook,
        status: data::ExecutionStatus::Waiting,
    })
}