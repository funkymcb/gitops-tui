use::std::io;
// use std::{io, thread, time::Duration, collections::HashMap};
// use tui::{
//     backend::CrosstermBackend,
//     widgets::{Widget, Block, Borders},
//     layout::{Layout, Constraint, Direction},
//     Terminal
// };
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };

mod config;

fn main() -> Result<(), io::Error> {
    let cfg = config::read();
    println!("{:#?}", cfg);
    // // setup terminal
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Block")
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;

    // thread::sleep(Duration::from_millis(5000));

    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    Ok(())
}
