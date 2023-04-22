use::std::io;
use::git2::Repository;

mod config;
mod git;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();
    let repo = match Repository::open(&cfg.clusters[0].path) {
        Ok(repo) => repo,
        Err(e) => panic!("Could not open repository: {}", e),
    };
    let commits = git::get_commits(&repo);

    println!("{:#?}", commits);

    Ok(())
}
