use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use crossterm::{execute, event};
use crossterm::event::{EnableMouseCapture, Event, KeyCode, DisableMouseCapture};
use git2::Repository;
use tui::layout::{Layout, Direction, Constraint};
use tui::style::{Style, Modifier, Color};
use tui::text::Spans;
use tui::{Terminal, Frame};
use tui::backend::{CrosstermBackend, Backend};
use tui::widgets::{ListItem, List, Block, Borders};
use std::{io, vec};

mod commits;
use commits::CommitList;

struct App {
    commits: CommitList,
}

impl App {
    fn new(commits: Vec<(String, bool)>) -> App {
        App {
           commits: CommitList::with_items(commits),
        }
    }
}

pub fn init(commits: (Vec<String>, Repository)) {
    // setup terminal
    enable_raw_mode()
        .expect("unable to initialize raw mode terminal");

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .expect("unable to execute terminal");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .expect("unable to initialize tui");

    let mut stateful_commits = Vec::new();
    for commit in commits.0 {
        stateful_commits.push((commit, false))
    }

    // create app and run it
    let app = App::new(stateful_commits);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()
        .expect("could not disable raw mode");

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
        .expect("unable to quit tui");

    terminal.show_cursor()
        .expect("could not show cursor");

    if let Err(err) = res {
        print!("{:?}", err)
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App,) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Enter => app.commits.toggle(),
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

    let commits: Vec<ListItem> = app.commits.items.iter().map(|i| {
        let lines = vec![Spans::from(i.0.as_str())];
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

    // TODO think about how to get diffs of selected commits herer
    let diffs: Vec<ListItem> = Vec::new();
    let diff_list = List::new(diffs)
        .block(Block::default().borders(Borders::ALL).title("Diff Tree"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
            )
        .highlight_symbol(">> ");

    f.render_stateful_widget(commit_list, chunks[0], &mut app.commits.state);
    f.render_stateful_widget(diff_list, chunks[1], &mut app.commits.state);
}
