use crate::rule::Rules;
use config::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Config represents the configuration of commitlint.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Settings {
    /// Rules represents the rules of commitlint.
    pub rules: Rules,
}
impl Settings {
    #[cfg(feature = "config")]
    const DEFAULT_CONFIG_FILENAME: &str = ".commitlintrc";
    #[cfg(feature = "config")]
    pub fn new(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut conf_builder = Config::builder();
        if let Some(path) = path {
            conf_builder =
                conf_builder.add_source(File::with_name(path.to_str().unwrap()).required(true));
        }
        conf_builder = conf_builder
            .add_source(File::with_name(Self::DEFAULT_CONFIG_FILENAME).required(false))
            .add_source(Environment::with_prefix("CL"));
        let conf = conf_builder.build()?;

        conf.try_deserialize()
    }
}
impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_yaml::to_string(&self).unwrap();
        write!(f, "{}", s)
    }
}

pub async fn load(path: Option<PathBuf>) -> Result<Settings, ConfigError> {
    Settings::new(path)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "schemars")]
    #[test]
    fn generate_json_schema() {
        use crate::settings::Settings;
        use insta::assert_yaml_snapshot;
        use std::fs;

        let config_schema = schemars::schema_for!(Config);
        let config_schema_json = serde_json::to_string_pretty(&config_schema).unwrap();
        assert_yaml_snapshot!(config_schema);
        fs::write("json-schema/config.json", config_schema_json).unwrap();
    }
}
