use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process::Command;

fn main() {
    let _ = uptime();
    let _ = cpu_temp();
    let _ = cpu_load_average();
}

fn uptime() {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("failed to get uptime");

    println!("{}", String::from_utf8_lossy(&uptime.stdout));
}

fn cpu_temp() -> Result<()> {
    let file = File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let regex = Regex::new(r"\d+").unwrap();
    let res = regex
        .find(&contents)
        .unwrap()
        .as_str()
        .parse::<f32>()
        .unwrap();

    // println!("{:#?}", res);

    println!("CPU Temp: {} °C", res / 1000_f32);

    return Ok(());
}

fn parse_load_average(load_average: &String) -> Option<(f64, f64, f64, i32, i32, i32)> {
    let regex = Regex::new(r"[\d\.]+").unwrap();
    let mut caps = regex
        .find_iter(load_average)
        .map(|x| x.as_str().parse::<f64>().ok());

    // println!("{}", load_average);

    let avg1: f64 = caps.next()??;
    let avg5: f64 = caps.next()??;
    let avg15: f64 = caps.next()??;
    let active_procs: i32 = caps.next()?? as i32;
    let total_procs: i32 = caps.next()?? as i32;
    let last_proc: i32 = caps.next()?? as i32;

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
