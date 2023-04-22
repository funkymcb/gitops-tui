use::std::io;

mod config;
mod git;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();

    let git = match git::init(&cfg.clusters[0].path) {
        Ok(git) => git,
        Err(e) => panic!("Could not initialize git handling {:?}", e),
    };

    print!("{:?}", git);

    Ok(())
}
