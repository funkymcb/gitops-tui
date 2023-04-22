use git2::{Error, Repository, Sort, Time};

#[derive(Debug)]
pub struct ExtendedCommit {
    pub id: String,
    pub summary: String,
    pub author: String,
    pub date: Time,
}

pub fn init(path: &String) {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Could not open repository {}", e),
    };
    let commits = get_commits(&repo);

    println!("{:#?}", commits);
}

pub fn get_commits<'a>(repo: &'a Repository) -> Result<Vec<ExtendedCommit>, Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;

        let extd_commit = ExtendedCommit {
            id: commit.id().to_string(),
            summary: commit.summary().unwrap_or("").to_string(),
            author: commit.author().to_string(),
            date: commit.author().when(),
        };

        commits.push(extd_commit);
    }

    Ok(commits)
}
