use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

type ID = String;
type Timestamp = String;

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
    pub target_webhook: Url,
    pub response_webhook: Url,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledExecutionStatus {
    pub id: ID,
    pub scheduled_timestamp: Timestamp,
    pub target_webhook: Url,
    pub response_webhook: Url,
    pub status: ExecutionStatus,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResponse {
    id: ID,
    scheduled_timestamp: Timestamp,
    target_webhook: Url,
    status: u8,
    headers: HashMap<String, String>,
    body: String,
}
