use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use crossterm::{execute, event};
use crossterm::event::{EnableMouseCapture, Event, KeyCode, DisableMouseCapture};
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
    items: CommitList,
}

impl App {
    fn new(commits: Vec<(String, bool)>) -> App {
        App {
           items: CommitList::with_items(commits),
        }
    }
}

pub fn init(commits: Vec<String>) {
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
    for commit in commits {
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
                KeyCode::Enter => app.items.toggle(),
                KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
                KeyCode::Down | KeyCode::Char('j') => app.items.next(),
                KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let commit_list = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app.items.items.iter().map(|i| {
        let lines = vec![Spans::from(i.0.as_str())];
        if i.1 {
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::LightGreen))
        } else {
            ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Black))
        }
    })
    .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Commits"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
            )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, commit_list[0], &mut app.items.state);
}
