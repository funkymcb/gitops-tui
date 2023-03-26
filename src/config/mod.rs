mod config {
    struct Stage {
        name: String,
        weight: i8,
    }

    struct Cluster {
        stage: Stage,
        path: String,
        repo: String,
    }

    pub struct Config {
        clusters: Vec<Cluster>,
    }
}
