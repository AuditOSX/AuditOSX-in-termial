use std::{fs::File, io::Write, process::Command};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

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
            serial_number: get_serial_number().expect("REASON"),
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

    pub fn save_to_json(&self, path: &str) -> std::io::Result<()> {
        // Check if the last character of the path is a slash (/) or backslash (\) depending on your OS
        // and append one if it's missing. This example uses `/` which is common for Unix-like systems.
        let separator = if path.ends_with('/') { "" } else { "/" };
        let file_name = format!("{}.json", &self.serial_number.replace(" ", "_")); // Replace spaces to avoid issues in filenames
        let full_path = format!("{}{}{}", path, separator, file_name);

        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(&full_path)?;
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

fn get_serial_number() -> Result<String, &'static str> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("cmd")
            .args(&["/C", "wmic bios get serialnumber"])
            .output()
            .map_err(|_| "Failed to execute command")?;

        let output_str = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse output")?;
        for line in output_str.lines() {
            if !line.starts_with("SerialNumber") && !line.is_empty() {
                return Ok(line.trim().to_string());
            }
        }
        Err("Serial number not found")
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("dmidecode -s system-serial-number")
            .output()
            .map_err(|_| "Failed to execute command")?;

        let output_str = std::str::from_utf8(&output.stdout).map_err(|_| "Failed to parse output")?;
        println!("{:?}", output);
        Ok(output_str.trim().to_string())
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ioreg -l | grep IOPlatformSerialNumber")
            .output()
            .map_err(|_| "Failed to execute command")?;

        let output_str = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse output")?;
        for line in output_str.lines() {
            if line.contains("IOPlatformSerialNumber") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() > 1 {
                    return Ok(parts[1].trim_matches(' ').trim_matches('"').to_string());
                }
            }
        }
        Err("Serial number not found")
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported OS")
    }
}