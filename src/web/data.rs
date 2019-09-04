#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type ID = String;
type Timestamp = String;
type Webhook = String;

#[derive(Deserialize, Debug)]
pub struct ScheduleExecutionPayload {
    pub scheduledTimestamp: Timestamp,
    pub targetWebhook: Webhook,
    pub responseWebhook: Webhook,
}

#[derive(Serialize, Debug)]
pub struct ScheduledExecutionStatus {
    pub id: ID,
    pub scheduledTimestamp: Timestamp,
    pub targetWebhook: Webhook,
    pub responseWebhook: Webhook,
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct ExecutionResponse {
    id: ID,
    scheduledTimestamp: Timestamp,
    targetWebhook: Webhook,
    status: u8,
    headers: HashMap<String, String>,
    body: String,
}
