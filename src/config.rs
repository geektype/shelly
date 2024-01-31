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
        if args.len() != 2 {
            return Err(ConfigError::MissingPrompt);
        }

        let prompt = args[1].clone();
        Ok(Self { api_key, prompt })
    }
}