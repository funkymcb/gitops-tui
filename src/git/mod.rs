use git2::{Commit, Error, Repository, Sort};

pub fn init(path: &String) {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Could not open repository {}", e),
    };
    let commits = get_commits(&repo);

    println!("{:#?}", commits);
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
