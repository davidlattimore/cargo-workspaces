use crate::utils::{Error, Result};

use serde::Deserialize;
use serde_json::{from_value, Value};

#[derive(Deserialize, Default)]
struct MetadataWorkspaces {
    pub workspaces: Option<Config>,
}

#[derive(Deserialize, Default, Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct Config {
    pub independent: Option<bool>,
}

impl Config {
    pub fn new(value: &Value) -> Result<Self> {
        from_value::<Option<MetadataWorkspaces>>(value.clone())
            .map_err(Error::BadMetadata)
            .map(|v| v.unwrap_or_default().workspaces.unwrap_or_default())
    }
}
