use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralCfg,
    pub clusters: Vec<Cluster>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "general")]
pub struct GeneralCfg {
    #[serde(rename = "time-format")]
    pub time_format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub stage: Stage,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub weight: i8,
}
