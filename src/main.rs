use::std::io;

mod config;
mod git;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();

    for repo in cfg.clusters {
        git::init(&repo.path);
    }

    Ok(())
}
