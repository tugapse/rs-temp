use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct SensorData {
    pub label: String,
    pub current: Option<f32>,
    pub high: Option<f32>,
    pub critical: Option<f32>,
}
