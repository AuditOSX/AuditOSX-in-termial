use chrono::{DateTime, Utc};
use sysinfo::System;

#[derive(Debug)] // Automatically derive Debug implementation
pub struct SysResume {
    pub computer_name: Option<String>,
    pub domain_name: Option<String>,
    pub site_name: String,
    pub roles: Vec<String>,
    pub description: String,
    pub operating_system: Option<String>,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub asset_tag: String,
    pub num_processors: usize,
    pub processor_description: String,
    pub total_memory: u64,
    pub total_hard_drive: String,
    pub display: String,
    pub bios_version: String,
    pub user_account: String,
    pub system_uptime: u64,
    pub local_time: DateTime<Utc>,
}

impl SysResume {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SysResume {
            computer_name: Some(System::name().unwrap_or_default()),
            domain_name: Some(System::host_name().unwrap_or_default()),
            site_name: "Replace this with actual data".to_string(),
            roles: vec!["Replace this with actual data".to_string()],
            description: "Replace this with actual data".to_string(),
            operating_system: Some(System::os_version().unwrap_or_default()),
            manufacturer: "Replace this with actual data".to_string(),
            model: "Replace this with actual data".to_string(),
            serial_number: System::distribution_id(),
            asset_tag: "Replace this with actual data".to_string(),
            num_processors: sys.cpus().len(),
            processor_description: "Replace this with actual data".to_string(),
            total_memory: sys.total_memory(),
            total_hard_drive: "Replace this with actual data".to_string(),
            display: "Replace this with actual data".to_string(),
            bios_version: "Replace this with actual data".to_string(),
            user_account: "Replace this with actual data".to_string(),
            system_uptime: System::uptime(),
            local_time: Utc::now(),
        }
    }

    // Getter
    pub fn get_sys_resume(&self) -> &Self {
        self
    }
}

impl Default for SysResume {
    fn default() -> Self {
        Self::new()
    }
}
