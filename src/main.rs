extern crate notify_rust;
extern crate sysinfo;

use notify_rust::Notification;
use std::collections::{
    hash_map::{self, Entry},
    HashMap,
};
use sysinfo::System;

fn notify(title: &str, body: &str) {
    _ = Notification::new().summary(title).body(body).show();
}

fn main() {
    let mut system = System::new_all();

    notify("Title", "Body copy");

    loop {
        system.refresh_all();

        // Get CPU usage
        let cpu_usage: f32 = system.global_cpu_info().cpu_usage();
        if cpu_usage > 90.0 {
            notify("High CPU Usage", &format!("CPU Usage: {:.2}%", cpu_usage));
        }

        let free_memory: u64 = system.free_memory();
        if free_memory < 1024 * 1024 * 100 {
            // Example threshold: 100 MB
            notify("Low Memory", &format!("Free Memory: {:.2} KB", free_memory));
        }

        let cores: usize = system.physical_core_count().unwrap();
        println!("cores: {:?}%\n", cores);

        let processes = system.processes();
        if processes.len() > 100 {
            let mut process_map: HashMap<&str, Vec<&str>> = HashMap::new();

            for (_pid, process) in processes {
                process_map.insert(process.name(), Vec::new());

                for (_pid, process_nested) in processes {
                    let current_entry: Entry<&str, Vec<&str>> =
                        process_map.entry(process_nested.name());
                    println!("Current Entry: {:?}", current_entry)

                    // insert instances count into relevant vec
                    //https://www.reddit.com/r/rust/comments/6d4no8/group_by_on_a_vect/
                }
            }

            notify(
                "High Process Count",
                &format!("Number of Processes: {}", processes.len()),
            );
        }

        // let available_memory: u64 = system.available_memory();
        // println!("Available Memory: {}\n", available_memory);

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

        // std::process::exit(0)

        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
