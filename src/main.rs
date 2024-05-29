mod ssh;
mod utils;
mod web;

use std::error::Error;

use std::net::TcpStream;
use std::process::Command;

use reqwest;

use tokio::io::AsyncBufReadExt;



fn scan_ports(target_ip: &str) -> Vec<u16> {
    println!("Scanning ports ");
    let mut open_ports = Vec::new();

    for port in 1..=10000 {
        let address = format!("{}:{}", target_ip, port);
        match TcpStream::connect(address) {
            Ok(_) => {
                println!("Port {} is open", port);
                open_ports.push(port);
            }
            Err(_) => {} // Ignore errors, assuming port is closed
        }
    }

    open_ports
}


pub fn get_ip_up() -> Result<(), Box<dyn Error>> {

    match utils::get_current_ip() {
        Some(ip) => {
            
            
            
            
            println!("IP: {}", ip);
            Ok(())
        
        },
        None => {
            Err("No interface found".into())
        },
    }

}

use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let ip_target = "192.168.1.122";
    let ip_srv = "51.77.193.65";
    let port = "8000";

    //let port_ouvert = scan_ports(ip_target); // donne un tableau de port ouvert

    //println!("Open ports: {:?}", port_ouvert);

    utils::get_rockyou(ip_srv, port)
        .await
        .expect("error : dl rockyou");

    get_ip_up();



/*
    match port_ouvert[1] {
        //22 => {
        //    ssh::ssh_bruteforce(ip_address, "bibli/").await;
        //},
        80 => {
            web::exploit_apache(ip_target, ip_srv).await;
        }

        _ => {}
    }
*/
}
