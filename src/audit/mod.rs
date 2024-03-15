use std::{fs::File, io::Write};

use chrono::{DateTime, Utc};
use sysinfo::{System,Disks};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] // Automatically derive Debug implementation
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
    pub total_hard_drive: u64,
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
            total_hard_drive: total_hard_drive_space(),
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

    // New method to save to JSON
    pub fn save_to_json(&self, file_path: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

impl Default for SysResume {
    fn default() -> Self {
        Self::new()
    }
}

fn total_hard_drive_space() -> u64 {
    let disks = Disks::new_with_refreshed_list();
    let mut tota_space_disks = 0u64;
    for disk in &disks {
        tota_space_disks += disk.total_space();
    }

    tota_space_disks
}
