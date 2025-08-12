use mldl_types::DeviceSpec;
use sysinfo::{Components, Disks, System};

pub fn collect_device_spec() -> DeviceSpec {
    let mut system = System::new_all();
    system.refresh_all();

    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    let os = System::name().unwrap_or_else(|| "unknown".to_string());

    let cpu_model = system
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .filter(|s| !s.is_empty());
    let cpu_cores = Some(system.cpus().len() as u32);

    let total_memory_gb = (system.total_memory() as f32) / (1024.0 * 1024.0);
    let memory_gb = Some(total_memory_gb);

    // GPU details not available via sysinfo; placeholders for later vendor-specific collectors
    let gpu_model = None;
    let gpu_driver = None;

    let _disks = Disks::new_with_refreshed_list();
    let _components = Components::new_with_refreshed_list();

    DeviceSpec {
        id: None,
        hostname,
        os,
        cpu_model,
        cpu_cores,
        memory_gb,
        gpu_model,
        gpu_driver,
    }
}


