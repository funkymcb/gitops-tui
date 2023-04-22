use::std::io;

mod config;
mod git;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();
    let log = git::log(cfg.clusters[0].path.clone());

    println!("{:#?}", log);

    Ok(())
}
