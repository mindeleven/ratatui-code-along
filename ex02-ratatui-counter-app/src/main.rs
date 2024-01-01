/// coding along with the tutorials at https://ratatui.rs/
/// Example 02, Counter App @ https://ratatui.rs/tutorials/counter-app/
/// 
/// Create a TUI application
/// the goal of this demonstration is to build a simple counter application
/// the counter should be increment or decrement when a key is pressed
/// (j for increment and k for decrement)
/// 
/// importing the necessary components from ratatui
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph
};
use std::io::Result;

fn main() -> Result<()> {
    // defining counter variable to track the "state" of the app
    let mut counter = 0;
    
    // using crossterm to set the terminal to raw mode
    // I guess crossterm comes with the prelude from ratatui ?
    // function needs to return an io::Result<()> to catch the error
    // like in the example at
    // https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    crossterm::terminal::enable_raw_mode()?;
    // then enter an alternate screen
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // creating an instance of a terminal backend with crossterm
    let mut terminal = Terminal::new(
        CrosstermBackend::new(std::io::stderr())
    )?;

    // TODO do something after terminal has been created
    // the main application loop
    // within the loop the app checks for user input,
    // updates the state and updates the display
    // Main application loop
    loop {
        // draw UI based on state
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(
                format!("Counter: {}", counter)), f.size()
            );
        })?;
        
        // update state based on user input
        
        // break from loop based on user input and/or state
        // checking for user input
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    _ => {},
                }
            }
        }
    }

    // disable raw mode for a clean exit, then exit the alternate screen
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    // then exit the alternate screen and returns to its original state
    crossterm::terminal::disable_raw_mode()?;

    println!("Alternate screen successfully exited");

    Ok(())
}
