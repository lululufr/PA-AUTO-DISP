mod ssh;
mod utils;
mod web;

use std::collections::HashMap;
use std::error::Error;

use std::net::{IpAddr, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use threadpool::ThreadPool;
use crate::utils::NetworkInterface;
use std::time::Duration;










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


    match utils::get_parc_ip().await {
        Ok(ips_ports) => {
            println!("{:?}",ips_ports);


            for ip in ips_ports.keys() {

                println!("ip: {}", ip);
                println!("port: {:?}", ips_ports.get(ip).unwrap());

            }



        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }



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




