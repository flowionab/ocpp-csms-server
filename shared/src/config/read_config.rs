use crate::config::Config;
use tokio::fs;
use tracing::error;

const CONFIG_FILE_PATHS: [&str; 2] = ["config/config.toml", "config.toml"];

pub async fn read_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let path = "config.toml";
    match find_paths().await? {
        Some(contents) => match toml::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                error!("error while parsing config file");
                eprintln!("{}", e);
                Err("Invalid config file".into())
            }
        },
        None => {
            let config = Config::default();
            fs::write(path, toml::to_string_pretty(&config)?).await?;
            Ok(config)
        }
    }
}

async fn find_paths() -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    for path in CONFIG_FILE_PATHS.iter() {
        if let Some(contents) = read_file(path).await? {
            return Ok(Some(contents));
        }
    }
    Ok(None)
}

async fn read_file(path: &str) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    match fs::try_exists(path).await? {
        true => {
            let contents = fs::read_to_string(path).await?;
            Ok(Some(contents))
        }
        false => Ok(None),
    }
}
