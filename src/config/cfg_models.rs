use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub weight: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub stage: Stage,
    pub path: String,
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub clusters: Vec<Cluster>,
}
