use::std::io;

mod config;
mod git;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();

    // TODO initialize all configured repos
    let git = match git::init(&cfg.clusters[0].path) {
        Ok(git) => git,
        Err(e) => panic!("Could not initialize git handler {:?}", e),
    };

    print!("{:?}", git);

    Ok(())
}
