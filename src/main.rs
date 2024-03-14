use sysinfo::{
    Components, Disks, Networks, System,
};
fn main() {
    println!("Hello, world!");
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("1) Résumé du Système :");
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    println!("System host boot time:   {:?}", System::boot_time());
    println!("System host long os name:   {:?}", System::long_os_version());
    println!("System host cpu arch:   {:?}", System::cpu_arch());
    println!("System host distribution id:   {:?}", System::distribution_id());
    println!("System host uptaime:   {:?}", System::uptime());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());


}

