use chrono::{DateTime, Utc};
use nvml_wrapper::Nvml;
use serde::Serialize;
use sysinfo::Components;

#[derive(Serialize, Debug)]
pub struct SensorData {
    pub label: String,
    pub current: Option<f32>,
    pub high: Option<f32>,
    pub critical: Option<f32>,
}

#[derive(Serialize, Debug)]
pub struct GpuReport {
    pub timestamp: DateTime<Utc>,
    pub gpus: Vec<SensorData>,
}

pub struct GpuMonitor {
    nvml: Option<Nvml>,
}

impl GpuMonitor {
    pub fn new() -> Self {
        Self {
            nvml: Nvml::init().ok(), 
        }
    }

    pub fn fetch(&mut self) -> GpuReport {
        let mut gpus = Vec::new();

        if let Some(nvml) = &self.nvml {
            if let Ok(device) = nvml.device_by_index(0) {
                let label = device.name().unwrap_or_else(|_| "NVIDIA GPU".into());
                let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).ok();

                gpus.push(SensorData {
                    label,
                    current: temp.map(|t| t as f32),
                    high: None,
                    critical: None,
                });
            }
        }

      
        // Linux often labels the Intel integrated GPU thermal zone as "acpitz" or "i915"
        let components = Components::new_with_refreshed_list();
        for component in &components {
            let lower_label = component.label().to_lowercase();
            
            // Catch Intel GPU identifiers, but ignore CPU cores
            if (lower_label.contains("acpitz") || lower_label.contains("i915") || lower_label.contains("gpu")) 
                && !lower_label.contains("core") && !lower_label.contains("package") 
            {
                gpus.push(SensorData {
                    label: "Intel UHD Graphics".to_string(), // Rename it so it looks clean in your CLI!
                    current: component.temperature(),
                    high: component.max(),
                    critical: component.critical(),
                });
                
                break; 
            }
        }

        GpuReport {
            timestamp: Utc::now(),
            gpus,
        }
    }
}