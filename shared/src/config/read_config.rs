use crate::config::Config;
use tokio::fs;
use tracing::error;

pub async fn read_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
    let path = "config.toml";
    match fs::try_exists(path).await? {
        true => {
            let contents = fs::read_to_string(path).await?;
            match toml::from_str(&contents) {
                Ok(config) => Ok(config),
                Err(e) => {
                    error!("error while parsing config file");
                    eprintln!("{}", e.to_string());
                    Err("Invalid config file".into())
                }
            }
        }
        false => {
            let config = Config::default();
            fs::write(path, toml::to_string_pretty(&config)?).await?;
            Ok(config)
        }
    }
}
