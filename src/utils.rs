use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest;
use reqwest::get;

use pnet::datalink::{self, NetworkInterface as PnetNetworkInterface};

use ipnetwork::IpNetwork;
use threadpool::ThreadPool;
use crate::utils;


pub(crate) fn subprocess_run(cmd: &str) -> String {

        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .arg(cmd)
                .output()
                .expect("Erreur lors de l'exécution de la commande.")
        } else {
            Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Erreur lors de l'exécution de la commande.")

        };
        //return
        //println!("{}", String::from_utf8_lossy(&output.stdout).to_string());
        String::from_utf8_lossy(&output.stdout).to_string()
}


pub async fn get_rockyou(ip_serv: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let bibli_path = current_dir.join("bibli");

    if !bibli_path.exists() {
        println!("[x] - Le dossier 'bibli' n'est pas présent. Création du dossier...");
        fs::create_dir_all(&bibli_path)?;
    } else {
        println!("[x] - Le dossier 'bibli' est déjà présent.");
    }

    let rockyou_path = current_dir.join("bibli").join("rck1.txt");

    match fs::metadata(&rockyou_path) {
        Ok(_) => {
            println!("[x] - Le fichier 'rockyou' est déjà présent.");
            return Ok(());
        }
        Err(_) => {
            println!("[x] - Les fichiers ne sont pas présent. Téléchargement...");
            let mut url = String::new();
            let mut cmp = 15;

            while cmp > 0 {
                url = format!("http://{}:{}/rck{}.txt", ip_serv, port, cmp);

                let response = get(url).await.expect("[x] - Erreur lors de la requête HTTP");

                let mut file = fs::File::create(format!("bibli/rck{}.txt", cmp))?;

                file.write_all(
                    &response
                        .bytes()
                        .await
                        .expect("[x] - Erreur lors de la lecture des données HTTP"),
                )?;

                cmp = cmp - 1;
            }

            Ok(())
        }
    }
}


#[derive(Debug)]
pub struct NetworkInterface {
    pub ip_addr: IpAddr,
    pub netmask: u8, // Changed to u8 to hold the CIDR value
}

pub fn get_current_ip() -> Option<NetworkInterface> {
    for iface in datalink::interfaces() {
        if !iface.is_up() || iface.is_loopback() {
            continue;
        }

        for ip_network in iface.ips {
            match ip_network.ip() {
                IpAddr::V4(ipv4_addr) => {
                    let netmask = ip_network.mask();
                    // Convert netmask to CIDR
                    let netmask_cidr = match netmask {
                        IpAddr::V4(netmask_v4) => ipv4_netmask_to_cidr(netmask_v4),
                        IpAddr::V6(_) => continue, // We skip IPv6 for simplicity
                    };
                    return Some(NetworkInterface {
                        ip_addr: IpAddr::from(ipv4_addr),
                        netmask: netmask_cidr,
                    });
                }
                IpAddr::V6(_) => continue,
            }
        }
    }
    None
}

// Helper function to convert IPv4 netmask to CIDR
fn ipv4_netmask_to_cidr(netmask: std::net::Ipv4Addr) -> u8 {
    let octets = netmask.octets();
    octets.iter().map(|&o| o.count_ones() as u8).sum()
}



pub(crate) async fn scan(ip: IpAddr, mask: u8)->Result<Vec<String>, Box<dyn Error>>{
    let network = match IpNetwork::new(ip, mask) {
        Ok(net) => net,
        Err(e) => {
            eprintln!("Error creating network: {}", e);
            return Err("Error creating network".into())
        }
    };

    let mut ips_up: Vec<String> = Vec::new();
    let mut scanned_ips = HashSet::new();  // HashSet pour stocker les IPs déjà scannées

    println!("[x] - Scan du subnet en cours ... ");
    for ip in network.iter() {
        if !scanned_ips.contains(&ip.to_string()) {  // Vérifier si l'IP a déjà été scannée
            if up_or_not(ip) {
                //println!("{} is UP", ip);
                ips_up.push(ip.to_string());
            } else {
                //println!("{} is DOWN", ip);
            }
            scanned_ips.insert(ip.to_string());  // Ajouter l'IP au HashSet
        }
    }
    println!("[x] - Scan terminé");
    Ok(ips_up)
}



pub fn up_or_not(mut ip: IpAddr) -> bool {
    //ip = ip.to_string().parse().unwrap();

    if cfg!(target_os = "windows") {
        let cmd= format!("ping {} -n 1 -w 1",ip);
        let sortie = subprocess_run(&*cmd);
        if sortie.contains("TTL"){ true }else{ false }
    } else {
        let cmd= format!("ping {} -c 1 -W 0.1",ip);
        let sortie = subprocess_run(&*cmd);
        if sortie.contains("ttl"){true }else{ false }
    }
}


fn scan_ports(target_ip: String) -> Vec<u16> {
    println!("  [...] - Scanning ports ");
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let pool = ThreadPool::new(400);  // Limitez le nombre de threads simultanés à 100

    for port in 2..=10000 {
        let open_ports = Arc::clone(&open_ports);
        let target_ip = target_ip.clone();

        pool.execute(move || {
            let address = format!("{}:{}", target_ip, port);
            let socket_addr: SocketAddr = address.parse().expect("Invalid address");
            match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(1)) {
                Ok(_) => {
                    println!("            - {} open", port);
                    let mut open_ports = open_ports.lock().unwrap();
                    open_ports.push(port);
                }
                Err(_) => {
                    // Si erreur, on ne fait rien. La connexion a échoué, donc le port est fermé.
                }
            }
        });
    }

    pool.join();  // Attendez que tous les threads soient terminés

    let open_ports = Arc::try_unwrap(open_ports).unwrap().into_inner().unwrap();
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

pub async fn get_parc_ip() -> Result<HashMap<String, Vec<u16>>, Box<dyn Error>>{
    let interface = get_net_int();
    match interface {
        Ok(NetworkInterface) => {
            println!("[x] - IP: {}", NetworkInterface.ip_addr);
            println!("[x] - mask: {:?}", NetworkInterface.netmask);

            match scan(NetworkInterface.ip_addr, NetworkInterface.netmask).await {
                Ok(ip_up) => {
                    //println!("{:?}", ip_up);

                    let mut ip_to_attack: HashMap<String, Vec<u16>> = HashMap::new();

                    for ip in ip_up {
                        if ip != "192.168.1.254" { // épargner ma box

                            println!("IP up: {}", ip);

                            let ports_ouverts = scan_ports(ip.clone());

                            ip_to_attack.insert(ip.to_string(), ports_ouverts);
                        }
                    }

                    //for (ip, ports) in &ip_to_attack {
                    //    println!("IP: {} - Ports ouverts: {:?}", ip, ports);
                    //}

                    return Ok(ip_to_attack);
                }
                Err(e) => {
                    eprintln!("[x] - Erreur lors du scan: {}", e);
                    Err("Erreur lors du scan".into())
                }
            }
        }
        Err(e) => {
            eprintln!("Pas d'interface : {}", e);
            Err("Erreur lors du scan".into())

        }
    }
}
