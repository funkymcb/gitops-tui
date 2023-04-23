use git2::{Error, Repository, Sort};
use chrono::prelude::*;
use crate::CONFIG;

#[derive(Debug)]
pub struct ExtendedCommit {
    pub id: String,
    pub summary: String,
    pub author: String,
    pub date: String,
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

        let date = convert_timestamp(commit.author().when().seconds());

        let extd_commit = ExtendedCommit {
            id: commit.id().to_string(),
            summary: commit.summary().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("unknown").to_string(),
            date,
        };

        commits.push(extd_commit);
    }

    Ok(commits)
}

fn convert_timestamp(ts: i64) -> String {
    let datetime = Local.timestamp_opt(ts, 0).unwrap();

    let date = datetime.format(CONFIG.general.time_format.as_str());
    let date_string = date.to_string();
    return date_string;
}
