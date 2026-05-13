pub mod cpu;
pub mod gpu;
pub mod sensors;

use clap::Parser;
use cpu::CpuMonitor;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, conflicts_with = "short")]
    json: bool,

    #[arg(long, conflicts_with = "json")]
    short: bool,

    #[arg(short, conflicts_with = "short")]
    j: bool,

    #[arg(short, conflicts_with = "json")]
    s: bool,


}

fn normal_output(cpu_monitor: &mut CpuMonitor, gpu_monitor: &mut Option<gpu::GpuMonitor>) {
    let report = cpu_monitor.fetch();
    
    // 1. Simple "Overall" check
    if let Some(overall) = report.overall {
        if let Some(temp) = overall.current {
            println!("CPU Overall Temp: {:.1}°C", temp);
            println!("{}", "-".repeat(55)); // Visual separator
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

            println!("{: <15} {: >5.1}°C    {: <15} {: >5.1}°C", label_l, t0, label_r, t1);
        } 
        else if chunk.len() == 1 {
            let t0 = chunk[0].current.unwrap_or(0.0);
            let label_l = format!("Core {}", left_idx);

            println!("{: <15} {: >5.1}°C", label_l, t0);
        }
    }

    if let Some(gpu_mon) = gpu_monitor {
        let gpu_report = gpu_mon.fetch();
        if !gpu_report.gpus.is_empty() {
            println!("{}", "-".repeat(55));
            for gpu in gpu_report.gpus {
                if let Some(temp) = gpu.current {
                    println!("{: <15} {: >5.1}°C", gpu.label, temp);
                }
            }
        }
    }
}


fn main() {
    let args = Args::parse();
    let mut cpu_monitor = CpuMonitor::new();
    let mut gpu_monitor = gpu::GpuMonitor::new();
    
    if args.json || args.short || args.j || args.s{
        let cpu_report = cpu_monitor.fetch();
        let gpu_report = gpu_monitor.as_mut().map(|m| m.fetch());
        
        if args.json || args.j {
            // Merge the reports into a single JSON struct for output
            #[derive(serde::Serialize)]
            struct CombinedReport<'a> {
                cpu: &'a cpu::CpuReport,
                gpu: Option<&'a gpu::GpuReport>,
            }
            let combined = CombinedReport {
                cpu: &cpu_report,
                gpu: gpu_report.as_ref(),
            };
            
            match serde_json::to_string_pretty(&combined) {
                Ok(json) => println!("{}", json),
                Err(err) => eprintln!("JSON Error: {}", err),
            }
        } else if args.short || args.s {
            let mut out = String::new();
            if let Some(overall) = cpu_report.overall.as_ref().and_then(|o| o.current) {
                out.push_str(&format!("CPU: {:.1}°C", overall));
            }
            let core_temps: Vec<String> = cpu_report.cores.iter()
                .filter_map(|c| c.current.map(|t| format!("{:.1}", t)))
                .collect();
            if !core_temps.is_empty() {
                if !out.is_empty() { out.push_str(" | "); }
                out.push_str(&format!("Cores: [{}]", core_temps.join(", ")));
            }
            
            if let Some(gr) = gpu_report {
                let gpu_temps: Vec<String> = gr.gpus.iter()
                    .filter_map(|g| g.current.map(|t| format!("{:.1}", t)))
                    .collect();
                if !gpu_temps.is_empty() {
                    if !out.is_empty() { out.push_str(" | "); }
                    out.push_str(&format!("GPUs: [{}]", gpu_temps.join(", ")));
                }
            }
            
            println!("{}", if out.is_empty() { "No sensor data".to_string() } else { out });
        }
        return;
    }
    normal_output(&mut cpu_monitor, &mut gpu_monitor);
    
}
