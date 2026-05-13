use chrono::{DateTime, Utc};
use serde::Serialize;
use sysinfo::Components;
use regex::Regex;

use crate::sensors::SensorData;

#[derive(Serialize, Debug)]
pub struct CpuReport {
    pub timestamp: DateTime<Utc>,
    pub overall: Option<SensorData>,
    pub cores: Vec<SensorData>,
}

pub struct CpuMonitor {
    components: Components,
}

impl CpuMonitor {
    pub fn new() -> Self {
        Self {
            components: Components::new_with_refreshed_list(),
        }
    }

    pub fn fetch(&mut self) -> CpuReport {
        // Refresh component data on each cycle
        self.components.refresh( true);
       
        let mut overall = None;
        let mut cores = Vec::new();

        for component in &self.components {
            let label = component.label().to_string();
            
            let data = SensorData {
                label: label.clone(),
                current: component.temperature(),
                high: component.max(),
                critical: component.critical(),
            };

            if label.contains("Package") {
                overall = Some(data);
            } else if label.contains("Core") {
                cores.push(data);
            }
        }

        let re = Regex::new(r"\d+").unwrap();
        cores.sort_by_key(|c| {
            re.find(&c.label)
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(u32::MAX)
        });

        CpuReport {
            timestamp: Utc::now(),
            overall,
            cores,
        }
    }
}
