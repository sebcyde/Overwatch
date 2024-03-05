extern crate notify_rust;
extern crate sysinfo;

use notify_rust::Notification;
use sysinfo::System;

fn notify(title: &str, body: &str) {
    _ = Notification::new().summary(title).body(body).show();
}

fn main() {
    let mut system = System::new_all();

    notify("Title", "Body copy");

    // loop {
    system.refresh_all();

    // Get CPU usage
    let cpu_usage = system.global_cpu_info();
    println!("CPU Usage: {:?}%\n", cpu_usage);

    let one: u64 = system.free_memory();
    println!("Free Memory: {}\n", one);

    let cores: usize = system.physical_core_count().unwrap();
    println!("CPU Usage: {:?}%\n", cores);

    // let processes = system.processes();

    let available_memory: u64 = system.available_memory();
    println!("Available Memory: {}\n", available_memory);

    // for (pid, process) in processes {
    // println!("PID: {}", pid);
    // println!("Name: {}", process.name());

    // let disk_usage: DiskUsage = process.disk_usage();
    // println!(
    //     "read bytes   : new/total => {}/{}",
    //     disk_usage.read_bytes, disk_usage.total_read_bytes,
    // );
    // println!(
    //     "written bytes: new/total => {}/{}",
    //     disk_usage.written_bytes, disk_usage.total_written_bytes,
    // );
    //
    // println!("{} Memory Usage: {:?}\n", process.name(), process.memory());
    // }

    std::process::exit(0)

    // std::thread::sleep(std::time::Duration::from_secs(60));
    // }
}
