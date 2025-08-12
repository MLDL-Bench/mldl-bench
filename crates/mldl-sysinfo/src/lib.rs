use mldl_types::HardwareSummary;
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct BasicSysInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub total_memory_kib: u64,
    pub used_memory_kib: u64,
    pub cpu_count: usize,
}

pub fn basic_sys_info() -> BasicSysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown OS".into());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown Kernel".into());
    
    let total_memory_kib = sys.total_memory();
    let used_memory_kib = sys.used_memory();
    let cpu_count = sys.cpus().len();

    BasicSysInfo {
        os_name,
        kernel_version,
        total_memory_kib,
        used_memory_kib,
        cpu_count,
    }
}

pub fn hardware_summary() -> HardwareSummary {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_model = sys
        .cpus()
        .get(0)
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".into());

    // GPU detection varies by OS and drivers. Stub for now.
    let gpu_model = None;

    let total_ram_gb = (sys.total_memory() / 1024 / 1024) as u16;
    let os = System::name().unwrap_or_else(|| "Unknown OS".into());

    HardwareSummary {
        cpu_model,
        gpu_model,
        ram_gb: total_ram_gb,
        os,
    }
}

// #[cfg(any(target_os="windows", target_os="linux"))]
// pub fn cuda_driver_version() -> Result<String, ()> {
//     unsafe {
//         // from cuda-driver-sys: cuDriverGetVersion
//         let mut v: std::os::raw::c_int = 0;
//         let status = cuda_driver_sys::cuDriverGetVersion(&mut v as *mut _);
//         if status == cuda_driver_sys::CUresult::CUDA_SUCCESS {
//             // CUDA versioning uses 12010 -> 12.1
//             let major = v / 1000;
//             let minor = (v % 1000) / 10;
//             Ok(format!("{}.{}", major, minor))
//         } else {
//             Err(())
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_info_collects(){
        let info = basic_sys_info();
        assert!(info.total_memory_kib > 0);
        assert!(info.used_memory_kib > 0);
        assert!(info.cpu_count > 0);
    }

    #[test]
    fn hardware_summary_collects() {
        let summary = hardware_summary();
        assert!(!summary.cpu_model.is_empty());
        assert!(summary.ram_gb > 0);
        assert!(!summary.os.is_empty());
    }
}
