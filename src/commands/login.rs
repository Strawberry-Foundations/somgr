use std::fs;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration};
use serde_json::Value;

use stblib::colors::{BLACK, BLUE, BOLD, C_RESET, GREEN, RED, RESET, YELLOW};
use crate::statics::STRAWBERRY_ID_API;

#[derive(Debug, Default, Clone)]
pub struct StrawberryId {
    pub email: String,
    pub full_name: String,
    pub profile_picture: String,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    token: String,
}

impl StrawberryId {
    fn serializer(&self, text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }

    pub async fn login(&mut self, code: String) -> eyre::Result<&Self> {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            let response = reqwest::get(format!("{STRAWBERRY_ID_API}api/oauth/callback?code={code}")).await?;
            let body = response.text().await?;

            if let Ok(data) = self.serializer(body.as_str()) {
                if data["data"]["status"] != "Invalid Code" && data["data"]["status"] != "Not authenticated" {
                    println!("{GREEN}{BOLD}Authentication successful{C_RESET}");

                    self.email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                    self.full_name = data["data"]["user"]["full_name"].as_str().unwrap().to_string();
                    self.profile_picture = data["data"]["user"]["profile_picture_url"].as_str().unwrap().to_string();
                    self.username = data["data"]["user"]["username"].as_str().unwrap().to_string();
                    self.token = data["data"]["user"]["token"].as_str().unwrap().to_string();

                    break
                }
            }

            interval.tick().await;
        }

        Ok(self)
    }
}

pub async fn login() -> eyre::Result<()> {
    println!("{BOLD}{GREEN}--- Strawberry ID Login ---{C_RESET}");

    let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/request")).await?;
    let code = if request.status().is_success() {
        match request.text().await {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{BOLD}{RED} ! {RESET} Error while requesting login code: {err}{C_RESET}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{BOLD}{RED} ! {RESET} Error while requesting login code: Server is not reachable{C_RESET}");
        std::process::exit(1);
    };

    println!("Go to {BOLD}{BLUE}{STRAWBERRY_ID_API}de/login/oauth_dialog/sbcloud?code={code}{C_RESET}");
    
    let mut auth = StrawberryId::default();
    let credentials = auth.login(code).await?;

    println!("{GREEN}{BOLD}Logged in as {} (@{})", credentials.full_name, credentials.username);

    if let Some(home_dir) = dirs::home_dir() {
        let config_dir = home_dir.join(".config").join("strawberry-id");
        let credentials_path = config_dir.join("credentials.yml");

        if !config_dir.exists() {
            if let Err(err) = fs::create_dir_all(&config_dir) {
                eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} {}{C_RESET}", err);
            }
        }

        if !credentials_path.exists() {
            let credentials = Credentials {
                username: credentials.username.clone(),
                token: credentials.token.clone(),
            };

            match serde_yaml::to_string(&credentials) {
                Ok(credentials_str) => {
                    if let Err(err) = fs::write(&credentials_path, credentials_str) {
                        eprintln!("{RED}{BOLD}Error while writing file:{RESET} {}{C_RESET}", err);
                    } else {
                        println!("{GREEN}{BOLD}Credentials saved successfully to {:?}{C_RESET}", credentials_path);
                    }
                }
                Err(err) => eprintln!("{RED}{BOLD}Error while serializing data:{RESET} {}{C_RESET}", err),
            }
        } else {
            println!("{YELLOW}{BOLD}credentials.yml already exists at {:?}{C_RESET}", credentials_path);
        }

    } else {
        eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} Home directory not found.{C_RESET}");
    }

    Ok(())
}