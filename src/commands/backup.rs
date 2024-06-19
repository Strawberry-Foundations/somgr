use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use stblib::colors::{BOLD, C_RESET, GREEN, RED, UNDERLINE, YELLOW};

use crate::args::ARGS;
use crate::commands;
use crate::commands::login::Credentials;
use crate::statics::STRAWBERRY_CLOUD_API;
use crate::utilities::{calc_percent, format_size, make_absolute_path, serializer};

pub async fn main() {
    let credentials = match Credentials::read() {
        Ok(creds) => creds,
        Err(..) => {
            println!("{RED}{BOLD}Please authenticate with your Strawberry ID before using somgr's Backup function{C_RESET}");
            return;
        }
    };

    match ARGS.subcommand.as_str() {
        "setup" => setup(),
        "restore" => {

        }
        "upload" => {

        }
        "add" => add(credentials).await,
        "remove" => {

        }
        "list" => list(credentials).await,
        "status" => status(credentials).await,
        _ => commands::help::help()
    }
}

pub fn setup() {
    let home_dir = env::var("HOME").unwrap();
    let config_dir = PathBuf::from(home_dir).join(".config/somgr");

    fs::create_dir_all(&config_dir).unwrap();

    let backup_file_path = config_dir.join("backup.yml");

    let content = r#"backup:
  - %HOME%/.bashrc
"#;

    let mut file = File::create(backup_file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    eprintln!("{GREEN}{BOLD}Configured StrawberryOS Backups{C_RESET}");
}

pub async fn status(credentials: Credentials) {
    #[derive(Default)]
    struct Status {
        storage_quota_max: u64,
        storage_quota_used: u64,
        storage_quota_left: i64,
    }

    let url = format!("{STRAWBERRY_CLOUD_API}/status/{}@{}", credentials.username, credentials.token);
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();

    let status = if let Ok(data) = serializer(body.as_str()) {
        if data["data"] != "Invalid credentials" && data["data"] != "Invalid username" {
            Status {
                storage_quota_max: data["data"]["storage_quota"]["max"].to_string().parse().unwrap(),
                storage_quota_used: data["data"]["storage_quota"]["used"].to_string().parse().unwrap(),
                storage_quota_left: data["data"]["storage_quota"]["left"].to_string().parse().unwrap(),
            }
        }
        else {
            eprintln!("{RED}{BOLD}Invalid credentials{C_RESET}");
            std::process::exit(1);
        }
    } else { Status::default() };

    println!("{GREEN}{BOLD}{UNDERLINE}Strawberry Cloud - Usage{C_RESET}");
    let percent = calc_percent(status.storage_quota_used, status.storage_quota_max);

    let progress_bar = ProgressBar::new(status.storage_quota_max);

    let template = String::from("{percent}% [{bar:40.cyan/blue}] {used}/{max} - {left} left")
        .replace("{used}", &format_size(status.storage_quota_used as i64).to_string())
        .replace("{max}", &format_size(status.storage_quota_max as i64).to_string())
        .replace("{left}", &format_size(status.storage_quota_left).to_string());

    progress_bar.set_style(
        ProgressStyle::with_template(template.as_str())
            .unwrap()
            .progress_chars("=>-")
    );

    progress_bar.set_position(status.storage_quota_used);

    if percent > 85.0 {
        println!("{YELLOW}{BOLD}Warning: You used over 85% of your storage quota ({percent}%).{C_RESET}")
    }
    else {
        println!();
    }
}

pub async fn list(credentials: Credentials) {
    #[derive(Deserialize)]
    struct Data {
        files: Vec<String>,
    }

    #[derive(Deserialize)]
    struct Root {
        data: Data,
    }

    let url = format!("{STRAWBERRY_CLOUD_API}/list/{}@{}/sbos.backups", credentials.username, credentials.token);
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();

    if let Ok(data) = serializer(body.as_str()) {
        if data["data"] == "Invalid credentials" || data["data"] == "Invalid directory" {
            eprintln!("{RED}{BOLD}Invalid credentials and/or invalid directory{C_RESET}");
            std::process::exit(1);
        }
    }
    else {
        eprintln!("{RED}{BOLD}Invalid data{C_RESET}");
        std::process::exit(1);
    }

    let root: Root = serde_json::from_str(body.as_str()).unwrap();
    let files: Vec<String> = root.data.files;



    println!("{GREEN}{BOLD}{UNDERLINE}Strawberry Cloud - Files{C_RESET}");

    if files.is_empty() {
        println!("It seems like you don't have any files in your cloud ...");
    }
    for file in files {
        println!("{}", file);
    }
}


pub async fn add(credentials: Credentials) {
    let client = reqwest::Client::new();

    let parser: Vec<String> = std::env::args().skip(2).collect();
    let file = parser.clone().first().unwrap().to_string();

    let file_path = make_absolute_path(file.as_str());
    let path = Path::new(&file_path);

    let filename = if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str
        } else {
            eprintln!("Invalid filename");
            std::process::exit(1)
        }
    } else {
        eprintln!("No filename found");
        std::process::exit(1)
    };

    println!("{}", path.to_str().unwrap());

    let url = format!("{STRAWBERRY_CLOUD_API}upload/{}@{}?filename={filename}", credentials.username, credentials.token);

    let file_content = std::fs::read(file_path).unwrap();

    let response = client.post(url)
        .header("Content-Type", "multipart/form-data")
        .body(file_content)
        .send()
        .await.unwrap();

    println!("{}", response.text().await.unwrap());
}