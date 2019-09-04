use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type ID = String;
type Timestamp = String;
type Webhook = String;

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionStatus {
    Waiting,
    Executed,
    Cancelled,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleExecutionPayload {
    pub scheduled_timestamp: Timestamp,
    pub target_webhook: Webhook,
    pub response_webhook: Webhook,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledExecutionStatus {
    pub id: ID,
    pub scheduled_timestamp: Timestamp,
    pub target_webhook: Webhook,
    pub response_webhook: Webhook,
    pub status: ExecutionStatus,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResponse {
    id: ID,
    scheduled_timestamp: Timestamp,
    target_webhook: Webhook,
    status: u8,
    headers: HashMap<String, String>,
    body: String,
}
