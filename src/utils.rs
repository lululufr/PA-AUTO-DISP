use std::{fs};

use std::io::{Write};

use reqwest;
use reqwest::get;





pub async fn get_rockyou(ip_serv: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {

        let current_dir = std::env::current_dir()?;
        let bibli_path = current_dir.join("bibli");

        if !bibli_path.exists() {
            println!("Le dossier 'bibli' n'est pas présent. Création du dossier...");
            fs::create_dir_all(&bibli_path)?;
        } else {
            println!("Le dossier 'bibli' est déjà présent.");
        }


    let rockyou_path = current_dir.join("bibli").join("rck1.txt");

    match fs::metadata(&rockyou_path) {
        Ok(_) => {
            println!("Le fichier 'rockyou' est déjà présent.");
            return Ok(());
        }
        Err(_) => {
            println!("Les fichiers ne sont pas présent. Téléchargement...");
            let mut url = String::new();
            let mut cmp = 15;

            while cmp > 0 {
                url = format!("http://{}:{}/rck{}.txt", ip_serv, port, cmp);

                let response = get(url).await.expect("Erreur lors de la requête HTTP");

                let mut file = fs::File::create(format!("bibli/rck{}.txt",cmp))?;

                file.write_all(&response.bytes().await.expect("Erreur lors de la lecture des données HTTP"))?;

                cmp = cmp - 1;
            }

             Ok(())
        }
    }

}