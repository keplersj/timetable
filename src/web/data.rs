#![allow(non_snake_case)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ScheduleExecutionPayload {
    scheduledTimestamp: String,
    targetWebhook: String,
    responseWebhook: String,
}
