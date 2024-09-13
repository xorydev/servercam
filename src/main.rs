use std::io::Write;
use std::net;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
mod hardwaremon;
use hardwaremon::get_cpu_usage;
use hardwaremon::get_hostname;
use hardwaremon::get_memory_usage;

pub fn handle_connection(mut connection: net::TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let cpu_usage = get_cpu_usage()?;
        let ram_usage = get_memory_usage()?;
        let hostname = get_hostname()?;
        
        let response = format!("{}:{}:T{},F{}\n", hostname, cpu_usage.to_string(), ram_usage.0.to_string(), ram_usage.1.to_string());
        connection.write_all(response.as_bytes())?;
        sleep(Duration::from_secs(5));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stream = net::TcpListener::bind("0.0.0.0:2048")?;
    for connection in stream.incoming() {
        handle_connection(connection?)?;
    }
    Ok(())
}
