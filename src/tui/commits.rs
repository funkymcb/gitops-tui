use tui::widgets::ListState;

pub struct CommitList {
    pub state: ListState,
    pub items: Vec<(String, bool)>,
}

impl CommitList {
    pub fn with_items(items: Vec<(String, bool)>) -> CommitList {
        CommitList {
            state: ListState::default(), 
            items,
        }
    }

    pub fn toggle(&mut self) {
        let i = self.state.selected().unwrap();

        self.items[i].1 = !self.items[i].1;
        if self.items[i].1 {
            self.items[i].0.replace_range(0..4, "[x] ");
        } else {
            self.items[i].0.replace_range(0..4, "[ ] ");
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
