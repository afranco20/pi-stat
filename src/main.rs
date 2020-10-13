use std::process::Command;

fn main() {
    uptime();
}

fn uptime() {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("failed to get uptime");

    println!("{}", String::from_utf8_lossy(&uptime.stdout));
}
