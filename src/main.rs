use anyhow::Context;
use clap::Parser;
use reqwest::multipart::Form;
use serde::Deserialize;
use std::{path::PathBuf, process::exit};
use tokio::process::Command;

#[cfg(not(windows))]
const CONFIG_HOME: &str = "HOME";
#[cfg(not(windows))]
const CONFIG_NAME: &str = ".pingme.toml";
#[cfg(windows)]
const CONFIG_HOME: &str = "APPDATA";
#[cfg(windows)]
const CONFIG_NAME: &str = "pingme.toml";

fn get_config_path() -> anyhow::Result<PathBuf> {
    let home = std::env::var(CONFIG_HOME)?;
    Ok(PathBuf::from(home).join(CONFIG_NAME))
}

#[derive(Deserialize)]
struct Config {
    app_token: String,
    user_token: String,
}

#[derive(Deserialize)]
struct Response {
    status: i64,
}

#[derive(Parser, Debug)]
struct PingMe {
    #[arg(short = 'C', long = "config")]
    config: Option<String>,
    #[arg(short = 'c', long = "command")]
    command: bool,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = PingMe::parse();

    let config_path = match args.config {
        Some(path) => PathBuf::from(path),
        None => get_config_path()?,
    };
    let config: Config = toml::from_str(&String::from_utf8_lossy(&std::fs::read(config_path)?))?;

    let message = if args.command {
        if args.args.len() < 1 {
            panic!("must provide at least 1 argument when using -c");
        }
        let cmd_str = args.args.join(" ");
        let cmd_status = Command::new(args.args[0].clone())
            .args(&args.args.as_slice()[1..])
            .status()
            .await?;
        if cmd_status.success() {
            format!("command '{}' succeeded", &cmd_str)
        } else {
            format!(
                "command '{}' failed with exit code {}",
                &cmd_str,
                cmd_status.code().unwrap_or_default()
            )
        }
    } else {
        args.args.join(" ")
    };

    if message.is_empty() {
        eprintln!("message cannot be empty");
        exit(1);
    }

    let client = reqwest::Client::new();
    let form = Form::new()
        .text("token", config.app_token)
        .text("user", config.user_token)
        .text("message", message);
    let res = client
        .post("https://api.pushover.net/1/messages.json")
        .multipart(form)
        .send()
        .await?;

    let res_bytes = res.bytes().await?.to_vec();
    let res: Response =
        serde_json::from_slice(&res_bytes).context("failed to deserialize response json")?;
    if res.status == 1 {
        eprintln!("SUCCESS: sent ping");
    } else {
        eprintln!("ERROR: got status value other than 1 from API");
        eprintln!("response contents: {}", String::from_utf8_lossy(&res_bytes));
    }
    Ok(())
}
