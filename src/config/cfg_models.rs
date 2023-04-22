use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Stage {
    name: String,
    weight: i8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cluster {
    stage: Stage,
    path: String,
    repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    clusters: Vec<Cluster>,
}
