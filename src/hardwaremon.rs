use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn get_memory_usage() -> io::Result<(u64, u64)> {
    let file = File::open("/proc/meminfo")?;
    let reader = BufReader::new(file);
    let mut total_memory = 0;
    let mut free_memory = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("MemTotal:") {
            total_memory = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("MemFree:") {
            free_memory = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if total_memory != 0 && free_memory != 0 {
            break;
        }
    }

    Ok((total_memory, free_memory))
}

pub fn get_cpu_usage() -> io::Result<f64> {
    let file = File::open("/proc/stat")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let values: Vec<u64> = line.split_whitespace()
        .skip(1)  // Skip "cpu" prefix
        .take(7)  // We need the first 7 values
        .map(|val| val.parse().unwrap())
        .collect();

    let total = values.iter().sum::<u64>();
    let idle = values[3];
    let usage = 100.0 * (1.0 - (idle as f64 / total as f64));

    Ok(usage)
}


pub fn get_hostname() -> io::Result<String> {
    let mut hostname = String::new();
    File::open("/etc/hostname")?.read_to_string(&mut hostname)?;
    hostname = hostname.trim().to_string();
    Ok(hostname)
}
