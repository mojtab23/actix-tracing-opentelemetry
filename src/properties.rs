use crate::user::UserProperties;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct AppProperties {
    pub debug: bool,
    pub user: UserProperties,
}

impl AppProperties {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("resources/properties/default"))?;

        // Add in the current environment file
        // Default to 'dev' env
        // Note that this file is _optional_
        let env = env::var("PROFILE").unwrap_or_else(|_| "dev".into());
        s.merge(File::with_name(&format!("resources/properties/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("resources/properties/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // You may also programmatically change settings
        // s.set("database.url", "postgres://")?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
