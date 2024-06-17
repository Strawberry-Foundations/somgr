use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
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
        "restore" => {

        }
        "upload" => {

        }
        "add" => {

        }
        "remove" => {

        }
        "list" => {

        }
        "status" => status(credentials).await,
        _ => commands::help::help()
    }
}

#[derive(Default)]
struct Status {
    storage_quota_max: u64,
    storage_quota_used: u64,
    storage_quota_left: i64,
}

pub async fn status(credentials: Credentials) {
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


pub async fn add(credentials: Credentials) -> Result<(), reqwest::Error> {
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
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}