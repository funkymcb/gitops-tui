pub mod cfg_models;
use cfg_models::Config;

pub fn read() -> Config {
    let f = std::fs::File::open("config.yaml").expect("Could not open config file");
    let cfg: Config = serde_yaml::from_reader(f).expect("Could not read config values");
    return cfg
}
