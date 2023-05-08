use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use git2::Repository;
use std::{io, vec};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::{Frame, Terminal};

mod commits;
use commits::CommitList;

use crate::git::ExtendedCommit;

struct App {
    commits: CommitList,
    repo: Repository,
}

impl App {
    fn new(commits: Vec<(ExtendedCommit, bool)>, repo: Repository) -> App {
        App {
            commits: CommitList::with_items(commits),
            repo,
        }
    }
}

static mut DIFF_TREE: Vec<ListItem> = Vec::new();

pub fn init(commits: (Vec<ExtendedCommit>, Repository)) {
    // setup terminal
    enable_raw_mode().expect("unable to initialize raw mode terminal");

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("unable to execute terminal");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("unable to initialize tui");

    let mut stateful_commits = Vec::new();
    for commit in commits.0 {
        stateful_commits.push((commit, false))
    }

    // create app and run it
    let app = App::new(stateful_commits, commits.1);
    let res = run_app(&mut terminal, app);

    disable_raw_mode().expect("could not disable raw mode");

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("unable to quit tui");

    terminal.show_cursor().expect("could not show cursor");

    if let Err(err) = res {
        print!("{:?}", err)
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        // TODO seperate list movement keys (currently they work for all lists in parallel)
        // maybe switch focused list by pressing ctrl+arrow (ctrl+vim-key)
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Enter => app.commits.toggle(&app.repo),
                KeyCode::Left | KeyCode::Char('h') => app.commits.unselect(),
                KeyCode::Down | KeyCode::Char('j') => app.commits.next(),
                KeyCode::Up | KeyCode::Char('k') => app.commits.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());

    let commits: Vec<ListItem> = app
        .commits
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0.display.as_str())];
            if i.1 {
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::LightGreen))
            } else {
                ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
            }
        })
        .collect();

    let commit_list = List::new(commits)
        .block(Block::default().borders(Borders::ALL).title("Commits"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    unsafe {
        let diff_list = List::new(DIFF_TREE.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Diff Tree"))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(diff_list, chunks[1], &mut app.commits.state);
    }

    f.render_stateful_widget(commit_list, chunks[0], &mut app.commits.state);
}
