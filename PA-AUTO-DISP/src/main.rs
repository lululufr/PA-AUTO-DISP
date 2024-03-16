use std::process::Command;
use std::net::{TcpStream, SocketAddr};

fn ping_ip(ip_address: &str) -> Result<(), String> {
    let command = Command::new("ping")
        .arg("-c")
        .arg("4") // Nombre de paquets à envoyer
        .arg(ip_address)
        .output()
        .map_err(|e| format!("Failed to execute ping command: {}", e))?;

    if command.status.success() {
        println!("Ping successful to {}", ip_address);
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&command.stderr);
        Err(format!("Failed to ping {}: {}", ip_address, error_message))
    }
}

fn scan_ports(target_ip: &str) {
    println!("Scanning ports ");

    for port in 1..=650 {
        let address = format!("{}:{}", target_ip, port);
        match TcpStream::connect(address) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => {} // Ignore errors, assuming port is closed
        }
    }
}



fn main() {
    let ip_address = "127.0.0.1";

    match ping_ip(ip_address) {
        Ok(_) => scan_ports(ip_address),
        Err(_) => {}
    }

}
