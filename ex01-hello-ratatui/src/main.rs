/// coding along with the tutorials at https://ratatui.rs/
/// Example 01, Hello world @ https://ratatui.rs/tutorials/hello-world/
/// 
/// Create a TUI application
/// the goal of this demonstration is to replace the default console application code 
/// with a Ratatui application that displays a colored message the middle of the screen 
/// and waits for the user to press a key to exit
/// 
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    // Ratatui has a prelude module which re-exports the most used types and traits
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    // setting up and restoring the terminal
    // (1) entering the alternate screen
    stdout().execute(EnterAlternateScreen)?;
    // (2) enabling raw mode
    enable_raw_mode()?;
    // turns off input and output processing by the terminal
    // and gives app control over the screen
    // creating a backend and Terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // app clears the screen
    terminal.clear()?;

    // TODO main loop

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}
