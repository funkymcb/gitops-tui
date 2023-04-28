use std::str::from_utf8;

use git2::{Error, Object, ObjectType, Repository, Sort};
use git2::{Diff, DiffOptions, DiffStatsFormat};
use chrono::prelude::*;
use crate::CONFIG;

#[derive(Debug)]
pub struct ExtendedCommit {
    pub id: String,
    pub summary: String,
    pub author: String,
    pub date: String,
}

impl ExtendedCommit {
    fn to_string(self) -> String {
        let str = format!(
            "[ ] {}: {}: {}: {}",
            self.date,
            self.id,
            self.author,
            self.summary,
        );
        str
    }

    pub fn get_commit_diff(self, repo: &Repository) -> String {
        let diff_obj = get_commit_file_diff(repo, &self.id).unwrap();
        let stats = diff_obj.stats().unwrap();
        let buf = stats.to_buf(DiffStatsFormat::FULL, 80).unwrap();
        let diff = from_utf8(&*buf).unwrap().to_string();
        diff
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
            summary: String::from(commit.summary().unwrap_or("")),
            author: String::from(commit.author().name().unwrap_or("unknown")),
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

fn get_commit_file_diff<'a>(repo: &'a Repository, commit_id: &String) -> Result<Diff<'a>, Error> {
    let mut opts = DiffOptions::new();
    let tree = tree_to_treeish(repo, &commit_id.as_str()).unwrap().unwrap();
    repo.diff_tree_to_workdir_with_index(tree.as_tree(), Some(&mut opts))
}

fn tree_to_treeish<'a>(
    repo: &'a Repository,
    arg: &'a str
) -> Result<Option<Object<'a>>, Error> {
    let obj = repo.revparse_single(arg)?;
    let tree = obj.peel(ObjectType::Tree)?;
    Ok(Some(tree))
}
