use config::Config;
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

use crate::settings;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GitHubSettings {
    pub token: Option<String>,
    pub schedule: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct HNSettings {
    pub auth: Option<String>,
    pub schedule: String,
    pub disable_upvoted: Option<bool>,
    pub disable_favorites: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KarakeepSettings {
    pub auth: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RedditSettings {
    pub clientid: Option<String>,
    pub clientsecret: Option<String>,
    pub refreshtoken: Option<String>,
    pub username: Option<String>,
    pub schedule: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PinboardSettings {
    pub token: Option<String>,
    pub schedule: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Settings {
    pub hn: HNSettings,
    pub karakeep: KarakeepSettings,
    pub reddit: RedditSettings,
    pub github: GitHubSettings,
    pub pinboard: PinboardSettings,
}

impl Settings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .add_source(config::Environment::with_prefix("KS").separator("_"))
            .set_override("hn.schedule", "@daily")
            .unwrap()
            .set_override("reddit.schedule", "@daily")
            .unwrap()
            .set_override("github.schedule", "@daily")
            .unwrap()
            .set_override("pinboard.schedule", "@daily")
            .unwrap()
            .build()
            .unwrap();

        let mut settings = config
            .try_deserialize::<settings::Settings>()
            .expect("Failed to deserialize settings");

        settings.hn.disable_upvoted = read_optional_bool_env("KS_HN_DISABLE_UPVOTED")
            .expect("Failed to parse KS_HN_DISABLE_UPVOTED");
        settings.hn.disable_favorites = read_optional_bool_env("KS_HN_DISABLE_FAVORITES")
            .expect("Failed to parse KS_HN_DISABLE_FAVORITES");

        settings
    }
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();
pub fn get_settings() -> &'static Settings {
    SETTINGS.get_or_init(Settings::new)
}

fn read_optional_bool_env(name: &str) -> anyhow::Result<Option<bool>> {
    match env::var(name) {
        Ok(value) => parse_bool_value(&value)
            .map_err(|e| anyhow::anyhow!("invalid boolean value for {name}: {e}")),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(anyhow::anyhow!("failed to read {name}: {err}")),
    }
}

fn parse_bool_value(value: &str) -> anyhow::Result<Option<bool>> {
    value
        .parse::<bool>()
        .map(Some)
        .map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_bool_values() {
        assert_eq!(Some(true), parse_bool_value("true").unwrap());
        assert_eq!(Some(false), parse_bool_value("false").unwrap());
    }

    #[test]
    fn rejects_invalid_bool_values() {
        assert!(parse_bool_value("maybe").is_err());
    }
}
