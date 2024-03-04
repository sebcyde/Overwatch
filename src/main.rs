extern crate sysinfo;

use sysinfo::{DiskUsage, System};

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        // Get CPU usage
        let cpu_usage = system.global_cpu_info();
        println!("CPU Usage: {:?}%\n", cpu_usage);

        let one: u64 = system.free_memory();
        println!("Free Memory: {}n", one);

        let cores: usize = system.physical_core_count().unwrap();
        println!("CPU Usage: {:?}%\n", cores);

        let processes = system.processes();
        for (pid, process) in processes {
            println!("PID: {}", pid);
            println!("Name: {}", process.name());

            // let disk_usage: DiskUsage = process.disk_usage();
            // println!(
            //     "read bytes   : new/total => {}/{}",
            //     disk_usage.read_bytes, disk_usage.total_read_bytes,
            // );
            // println!(
            //     "written bytes: new/total => {}/{}",
            //     disk_usage.written_bytes, disk_usage.total_written_bytes,
            // );

            println!("Process Memory Usage: {:?}\n", process.memory());
        }

        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
