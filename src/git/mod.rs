use std::error;
use std::str::from_utf8;

use crate::CONFIG;
use chrono::prelude::*;
use git2::{Diff, DiffOptions, DiffStatsFormat};
use git2::{Error, Object, ObjectType, Repository, Sort};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct ExtendedCommit {
    pub author: String,
    pub date: String,
    pub display: String,
    pub id: String,
    pub summary: String,
}

impl ExtendedCommit {
    // TODO dont return string but some sort of Diff struct itself
    // for now we will parse the returned diffstats string... this is prone to error tho
    // need to find a way to list the files programatically utilising the git2 module
    pub fn get_diff(&self, repo: &Repository) -> Result<String, Box<dyn error::Error>> {
        let diff_obj = get_commit_file_diff(repo, &self.id)?;
        let stats = diff_obj.stats()?;
        let buf = stats.to_buf(DiffStatsFormat::FULL, 80)?;
        let diff = from_utf8(&*buf)?.to_string();
        // let stripped_diff = strip_diff_to_files(diff);
        Ok(diff)
    }
}

pub fn init(path: &String) -> Result<(Vec<ExtendedCommit>, Repository), Error> {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("Could not open repository {}", e),
    };

    let commit = get_commits(&repo)?;
    Ok((commit, repo))
}

pub fn get_commits<'a>(repo: &'a Repository) -> Result<Vec<ExtendedCommit>, Error> {
    let mut revwalk = repo.revwalk()?;

    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;

        let author = String::from(commit.author().name().unwrap_or("unknown"));
        let date = convert_timestamp(commit.author().when().seconds());
        let id = commit.id().to_string()[0..8].to_string();
        let summary = String::from(commit.summary().unwrap_or(""));
        let display = create_display(&id, &summary, &author, &date);

        let extd_commit = ExtendedCommit {
            author,
            date,
            display,
            id,
            summary,
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

fn create_display(id: &String, summary: &String, author: &String, date: &String) -> String {
    let str = format!("[ ] {}: {}: {}: {}", date, id, author, summary,);
    str
}

fn get_commit_file_diff<'a>(repo: &'a Repository, commit_id: &String) -> Result<Diff<'a>, Error> {
    let mut opts = DiffOptions::new();
    let tree = tree_to_treeish(repo, &commit_id.as_str())?.unwrap();
    repo.diff_tree_to_workdir(tree.as_tree(), Some(&mut opts))
}

fn tree_to_treeish<'a>(repo: &'a Repository, arg: &'a str) -> Result<Option<Object<'a>>, Error> {
    let obj = repo.revparse_single(arg)?;
    let tree = obj.peel(ObjectType::Tree)?;
    Ok(Some(tree))
}

fn strip_diff_to_files(diff_str: String) -> String {
    // TODO implement logic like in tests
    String::new()
}
