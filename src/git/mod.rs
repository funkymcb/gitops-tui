use git2::{Error, Repository, Sort};

pub fn log(path: String) -> Result<(), Error> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit.summary_bytes().unwrap_or_else(|| commit.message_bytes());
        println!("{}\t{}", commit.id(), String::from_utf8_lossy(message));
    }

    Ok(())
}
