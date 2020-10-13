use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process::Command;

fn main() {
    uptime();
    cpu_temp();
    let _ = cpu_load_average();
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

fn parse_load_average(load_average: &String) -> Option<(f32, f32, f32, i32, i32, i32)> {
    let regex = Regex::new(r"[\d\.]+").unwrap();
    let mut caps = regex.find_iter(load_average);

    let avg1 = caps.next().unwrap().as_str().parse().ok()?;
    let avg5 = caps.next().unwrap().as_str().parse().ok()?;
    let avg15 = caps.next().unwrap().as_str().parse().ok()?;
    let active_procs = caps.next().unwrap().as_str().parse().ok()?;
    let total_procs = caps.next().unwrap().as_str().parse().ok()?;
    let last_proc = caps.next().unwrap().as_str().parse().ok()?;

    return Some((avg1, avg5, avg15, active_procs, total_procs, last_proc));
}

fn cpu_load_average() -> Result<()> {
    let file = File::open("/proc/loadavg")?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let res = parse_load_average(&contents).unwrap();

    // println!("{:#?}", res);
    println!("{:.2} {:.2} {:.2}", res.0, res.1, res.2);

    return Ok(());
}
