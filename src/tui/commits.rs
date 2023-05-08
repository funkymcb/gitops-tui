use git2::Repository;
use tui::widgets::{ListItem, ListState};

use crate::git::ExtendedCommit;

use super::DIFF_TREE;

pub struct CommitList {
    pub state: ListState,
    pub items: Vec<(ExtendedCommit, bool)>,
}

impl CommitList {
    pub fn with_items(items: Vec<(ExtendedCommit, bool)>) -> CommitList {
        CommitList {
            state: ListState::default(),
            items,
        }
    }

    pub fn toggle(&mut self, repo: &Repository) {
        let i = self.state.selected().unwrap();

        self.items[i].1 = !self.items[i].1;

        if self.items[i].1 {
            self.items[i].0.display.replace_range(0..4, "[x] ");
        } else {
            self.items[i].0.display.replace_range(0..4, "[ ] ");
        }

        // TODO toggle logic for diffs... Remove item is commit is untoggled
        let diff = self.items[i]
            .0
            .get_diff(&repo)
            .unwrap_or(String::from("no diffs"));
        let list_item = ListItem::new(diff);
        unsafe {
            DIFF_TREE.push(list_item);
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
