use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub board_from_file: bool,
    pub board_path: String,
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = "config.toml";

    if let Ok(config_content) = std::fs::read_to_string(config_path) {
        let config: AppConfig = toml::from_str(&config_content)?;

        if validate_config(&config) {
            Ok(config)
        } else {
            get_fallback_config()
        }
    } else {
        get_fallback_config()
    }
}

fn validate_config(config: &AppConfig) -> bool {
    !config.board_path.is_empty()
}

fn get_fallback_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let fallback_config: AppConfig = AppConfig {
        board_from_file: false,
        board_path: "".to_string(),
    };
    Ok(fallback_config)   
}