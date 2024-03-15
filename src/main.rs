use auditax::audit::SysResume;

fn main() {
    let sys_resume = SysResume::default();
    println!("{:#?}", sys_resume);

    // Specify the path to save the JSON file
    let file_path = "system_resume.json";
    sys_resume.save_to_json(file_path).expect("Failed to save to JSON");
    println!("System resume saved to {}", file_path);
}
