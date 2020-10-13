use std::process::Command;

fn main() {
    uptime();
    cpu_temp();
}

fn uptime() {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("failed to get uptime");

    println!("{}", String::from_utf8_lossy(&uptime.stdout));
}

fn cpu_temp() {
    let temp = Command::new("cat")
        .arg("/sys/class/thermal/thermal_zone0/temp")
        .output()
        .expect("failed to get cpu temp");

    let val: String = String::from_utf8(temp.stdout).expect("not valid utf8");
    let val: i32 = val.trim().parse().expect("not a valid number");

    println!("CPU Temp: {} °C", val / 1000);
    // println!("{}", temp.status);
}
