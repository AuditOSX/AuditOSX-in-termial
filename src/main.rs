use sysinfo::{
    get_current_pid, Components, Disks, Networks, System
};

use auditax::audit::{self, SysResume};
fn main() {
    SysResume::new();
    println!("{:?}",get_sys_resume);

}

