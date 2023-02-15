use crate::types::WorkingStatus;
use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub response: PayloadResponse,
}

#[derive(Debug, Deserialize)]
pub struct PayloadResponse {
    pub result: PayloadResult,
}

#[derive(Debug, Getters, Deserialize)]
pub struct PayloadResult {
    working_hours: f64,
    average_remain_hours: f64,
    minimum_average_remain_hours: f64,
    status: WorkingStatus,
    rest_timestamp: Option<i64>,
}
