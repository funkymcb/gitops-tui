use git2::{Commit, Error, Repository, Sort};

pub fn init(path: &String) -> Result<(), Error> {
    let repo = Repository::open(path)?;
    let commits = get_commits(&repo);

    println!("{:#?}", commits);

    Ok(())
}

pub fn get_commits<'a>(repo: &'a Repository) -> Result<Vec<Commit<'a>>, Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        commits.push(commit);
    }

    Ok(commits)
}
