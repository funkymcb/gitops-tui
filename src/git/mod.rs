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

impl<'a> ExtendedCommit {
    fn to_string(self) -> String {
        let str = format!(
            "{}: {}: {}: {}",
            self.date,
            self.id,
            self.author,
            self.summary,
        );
        str
    }
}

pub fn init(path: &String) -> Vec<String> {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Could not open repository {}", e),
    };

    let commit_str = get_commits(&repo).unwrap();
    commit_str
}

pub fn get_commits<'a>(repo: &'a Repository) -> Result<Vec<String>, Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let id = commit.id().to_string()[0..8].to_string();
        let date = convert_timestamp(commit.author().when().seconds());

        let extd_commit = ExtendedCommit {
            id,
            summary: commit.summary().unwrap_or("").to_string(),
            author: commit.author().name().unwrap_or("unknown").to_string(),
            date,
        };

        commits.push(extd_commit.to_string());
    }

    Ok(commits)
}

fn convert_timestamp(ts: i64) -> String {
    let datetime = Local.timestamp_opt(ts, 0).unwrap();

    let date = datetime.format(CONFIG.general.time_format.as_str());
    let date_string = date.to_string();
    return date_string;
}
