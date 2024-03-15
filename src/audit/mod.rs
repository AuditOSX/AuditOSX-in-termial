use chrono::{DateTime, Utc};
use sysinfo::System;

pub struct SysResume {
    computer_name:Option<String>,
    domain_name: Option<String>,
    site_name: String,
    roles: Vec<String>,
    description: String,
    operating_system: Option<String>,
    manufacturer: String,
    model: String,
    serial_number: String,
    asset_tag: String,
    num_processors: usize,
    processor_description: String,
    total_memory: u64,
    total_hard_drive: String,
    display: String,
    bios_version: String,
    user_account: String,
    system_uptime: u64,
    local_time: DateTime<Utc>,
}

impl SysResume {

    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        SysResume {
            computer_name: System::name(),
            domain_name: System::host_name(),
            site_name: todo!(),
            roles: todo!(),
            description: todo!(),
            operating_system: System::os_version(),
            manufacturer: todo!(),
            model: todo!(),
            serial_number: System::distribution_id(),
            asset_tag: todo!(),
            num_processors: todo!(),
            processor_description: todo!(),
            total_memory:  sys.total_memory(),
            total_hard_drive: todo!(),
            display: todo!(),
            bios_version: todo!(),
            user_account: todo!(),
            system_uptime: System::uptime(),
            local_time: chrono::DateTime::default(),
        }
    }

    // Getter
    pub fn get_sys_resume(&self) -> &Self {
        self
    }
}

println!("=> system:");
// RAM and swap information:
//println!("total memory: {} bytes", sys.total_memory());
println!("used memory : {} bytes", sys.used_memory());
println!("total swap  : {} bytes", sys.total_swap());
println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
println!("1) Résumé du Système :");
//println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());
println!("System host boot time:   {:?}", System::boot_time());
println!("System host long os name:   {:?}", System::long_os_version());
println!("System host cpu arch:   {:?}", System::cpu_arch());
println!("System host distribution id:   {:?}", System::distribution_id());
//println!("System host uptaime:   {:?}", System::uptime());

// Number of CPUs:
println!("NB CPUs: {}", sys.cpus().len());

