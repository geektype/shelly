#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingApiKey,
    MissingPrompt,
}

pub struct Config {
    pub api_key: String,
    pub prompt: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let api_key = std::env::var("SHELLY_OPENAI_KEY").map_err(|_| ConfigError::MissingApiKey)?;

        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Err(ConfigError::MissingPrompt);
        }

        let prompt = args[1].clone();
        Ok(Self { api_key, prompt })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_success() {
        // Set environment variable for testing
        std::env::set_var("SHELLY_OPENAI_KEY", "test_api_key");

        // Set command line arguments
        let args = vec!["program_name".to_string(), "test_prompt".to_string()];

        let result = Config::load(args);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.api_key, "test_api_key");
        assert_eq!(config.prompt, "test_prompt");
    }

    #[test]
    fn test_load_config_missing_api_key() {
        // Unset the environment variable
        std::env::remove_var("SHELLY_OPENAI_KEY");
        let args = vec!["program_name".to_string(), "test_prompt".to_string()];

        let result = Config::load(args);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err, ConfigError::MissingApiKey);
    }

    #[test]
    fn test_load_config_missing_prompt() {
        // Set environment variable for testing
        std::env::set_var("SHELLY_OPENAI_KEY", "test_api_key");

        // Set command line arguments without prompt
        let args = vec!["program_name1".to_string()];

        let result = Config::load(args);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err, ConfigError::MissingPrompt);
    }
}