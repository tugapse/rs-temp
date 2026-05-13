pub mod cpu;
pub mod gpu;
pub mod sensors;

use clap::{Parser, ValueEnum};
use cpu::{CpuMonitor, CpuReport};
use gpu::{GpuMonitor, GpuReport};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum DeviceType {
    Cpu,
    Gpu,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, conflicts_with = "short")]
    json: bool,

    #[arg(short, long, conflicts_with = "json")]
    short: bool,

    #[arg(short, long, value_enum)]
    device: Option<DeviceType>,
}

// =====================================================================
// Display Formatters (The new extracted functions)
// =====================================================================

fn print_cpu_normal(report: &CpuReport) {
    if let Some(overall) = &report.overall {
        if let Some(temp) = overall.current {
            println!("CPU Overall Temp: {:.1}C", temp);
            println!("{}", "-".repeat(55));
        }
    }

    for (i, chunk) in report.cores.chunks(2).enumerate() {
        let left_idx = i * 2 + 1;
        let right_idx = i * 2 + 2;

        if chunk.len() == 2 {
            let t0 = chunk[0].current.unwrap_or(0.0);
            let t1 = chunk[1].current.unwrap_or(0.0);
            let label_l = format!("Core {}", left_idx);
            let label_r = format!("Core {}", right_idx);
            println!("{: <15} {: >5.1}C    {: <15} {: >5.1}C", label_l, t0, label_r, t1);
        } else if chunk.len() == 1 {
            let t0 = chunk[0].current.unwrap_or(0.0);
            let label_l = format!("Core {}", left_idx);
            println!("{: <15} {: >5.1}C", label_l, t0);
        }
    }
}

fn print_gpu_normal(report: &GpuReport, print_separator: bool) {
    if report.gpus.is_empty() {
        return; // Nothing to print
    }
    
    if print_separator {
        println!("{}", "-".repeat(55));
    }
    
    for gpu in &report.gpus {
        if let Some(temp) = gpu.current {
            println!("{: <15} {: >5.1}C", gpu.label, temp);
        }
    }
}

fn print_short_output(cpu_report: Option<&CpuReport>, gpu_report: Option<&GpuReport>) {
    let mut parts = Vec::new();

    if let Some(report) = cpu_report {
        if let Some(overall) = report.overall.as_ref().and_then(|o| o.current) {
            parts.push(format!("CPU: {:.1}C", overall));
        }
    }

    if let Some(report) = gpu_report {
        let gpu_temps: Vec<String> = report.gpus.iter()
            .filter_map(|g| g.current.map(|t| format!("{:.1}C", t)))
            .collect();
        if !gpu_temps.is_empty() {
            parts.push(format!("GPUs: [{}]", gpu_temps.join(", ")));
        }
    }
    
    println!("{}", if parts.is_empty() { "No data".to_string() } else { parts.join(" | ") });
}

fn print_json_output(cpu_report: Option<&CpuReport>, gpu_report: Option<&GpuReport>) {
    #[derive(serde::Serialize)]
    struct CombinedReport<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        cpu: Option<&'a CpuReport>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gpu: Option<&'a GpuReport>,
    }
    
    let combined = CombinedReport {
        cpu: cpu_report,
        gpu: gpu_report,
    };
    
    if let Ok(json) = serde_json::to_string_pretty(&combined) {
        println!("{}", json);
    }
}

// =====================================================================
// Core Logic Handlers
// =====================================================================

fn normal_output(cpu_monitor: &mut CpuMonitor, gpu_monitor: &mut GpuMonitor, filter: &Option<DeviceType>) {
    let show_cpu = filter.is_none() || filter == &Some(DeviceType::Cpu);
    let show_gpu = filter.is_none() || filter == &Some(DeviceType::Gpu);

    if show_cpu {
        let cpu_report = cpu_monitor.fetch();
        print_cpu_normal(&cpu_report);
    }

    if show_gpu {
        let gpu_report = gpu_monitor.fetch();
        // If we printed the CPU, we want a separator before the GPU section.
        // We only print the separator if the filter was None (meaning both are showing).
        print_gpu_normal(&gpu_report, filter.is_none());
    }
}

fn main() {
    let args = Args::parse();
    let mut cpu_monitor = CpuMonitor::new();
    let mut gpu_monitor = GpuMonitor::new();
    
    // Determine what needs to be fetched based on the device filter
    let show_cpu = args.device.is_none() || args.device == Some(DeviceType::Cpu);
    let show_gpu = args.device.is_none() || args.device == Some(DeviceType::Gpu);

    if args.json || args.short {
        // Only fetch what is explicitly requested to save processing power
        let cpu_report = if show_cpu { Some(cpu_monitor.fetch()) } else { None };
        let gpu_report = if show_gpu { Some(gpu_monitor.fetch()) } else { None };
        
        if args.json {
            print_json_output(cpu_report.as_ref(), gpu_report.as_ref());
        } else if args.short {
            print_short_output(cpu_report.as_ref(), gpu_report.as_ref());
        }
        return;
    }

    normal_output(&mut cpu_monitor, &mut gpu_monitor, &args.device);
}