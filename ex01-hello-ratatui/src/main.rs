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
    
    // within the main loop the application draws the ui 
    //and then handles occurring events
    loop {
        // drawing the terminal
        // draw() is the main interaction point of an app
        // draw() accepts a closure with a Frame as parameter
        terminal.draw(|frame| {
            // creating an area that is the full size of the terminal window
            let area = frame.size();
            // rendering a new Paragraph with white foreground text and a blue background
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    // white() and on_blue() are defined in the Stylize 
                    // extension trait as style shorthands
                    .white()
                    .on_blue(), 
                area);

        })?;

        // handling events
        // checking to see if any events have occurred
        // adding a small timeout to the event polling to ensure that the UI remains responsive
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                // check if event kind is Press for Windows
                if key.kind == KeyEventKind::Press 
                    && key.code == KeyCode::Char('q') 
                {
                    break;
                }
            }
        }
    }
    
    // (3) exiting the alternate screen
    stdout().execute(LeaveAlternateScreen)?;
    // (4) disabling raw mode
    // if app doesn’t disable raw mode before exit
    // on a Linux / macOS terminal type reset to get out
    disable_raw_mode()?;
    
    Ok(())
}
