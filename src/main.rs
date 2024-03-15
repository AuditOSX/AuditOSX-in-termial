use std::process;

use auditax::{
    privilege::check_privileged, 
    audit::SysResume};

fn main() {
    if !check_privileged() {
        println!("This program must be run with privileged access.");
        process::exit(1);
    }
    let sys_resume = SysResume::default();
    println!("{:#?}", sys_resume);

    // Specify the directory path where you want to save the JSON file
    let path = "."; // Update this path as needed
    sys_resume.save_to_json(path).expect("Failed to save to JSON");

    println!("System resume saved successfully.");
}