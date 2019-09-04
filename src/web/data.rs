#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ScheduleExecutionPayload {
    pub scheduledTimestamp: String,
    pub targetWebhook: String,
    pub responseWebhook: String,
}

#[derive(Serialize, Debug)]
pub struct ScheduledExecutionStatus {
    pub id: String,
    pub scheduledTimestamp: String,
    pub targetWebhook: String,
    pub responseWebhook: String,
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct ExecutionResponse {
    id: String,
    scheduledTimestamp: String,
    targetWebhook: String,
    status: u8,
    headers: HashMap<String, String>,
    body: String,
}
