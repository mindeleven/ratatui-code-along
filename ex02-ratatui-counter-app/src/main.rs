/// coding along with the tutorials at https://ratatui.rs/
/// Example 02, Counter App @ https://ratatui.rs/tutorials/counter-app/
/// 
/// Create a TUI application
/// the goal of this demonstration is to build a simple counter application
/// the counter should be increment or decrement when a key is pressed
/// (j for increment and k for decrement)
/// 
/// importing the necessary components from crossterm
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
/// importing the necessary components from ratatui
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::Paragraph
};

/// defining custom types and aliases
type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

// defining an App struct to encapsulate the application state
struct App {
    // current state of the counter
    counter: i64,
    // flag that indicates whether app should exit main loop
    should_quit:  bool,
}


fn main() -> Result<()> {
    // defining counter variable to track the "state" of the app
    // let mut counter = 0;
    /* 
    // STARTUP code (gets replaced with startup() function):
    // using crossterm to set the terminal to raw mode
    // I guess crossterm comes with the prelude from ratatui ?
    // function needs to return an io::Result<()> to catch the error
    // like in the example at
    // https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    crossterm::terminal::enable_raw_mode()?;
    // then enter an alternate screen
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    */
    startup()?;

    /* 
    // MAIN application code (gets replaced with run() function):
    
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
        // closure passed to the Terminal::draw() method must render the entire UI
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
                // widows sends key event twice, for KeyEventKind::Press and KeyEventKind::Release
                // so we've to make sure that key.kind is KeyEventKind::Press only
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        // 'j' adds 1 to the counter
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        // 'k' subtracts 1 to from counter
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        // 'q' breaks the app
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }
    */
    // (1st) we get the result of run() (the status)
    // then (2nd) we call shutdown() 
    // and after that (3rd) we unwrap() the result
    // be keeping this order, we ensure
    // that shutdown() is always called before the program exits

    // (1st)
    let status = run();

    /*
    // SHUTDOWN code (gets replaced with shutdown() function):
    // disable raw mode for a clean exit, then exit the alternate screen
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    // then exit the alternate screen and returns to its original state
    crossterm::terminal::disable_raw_mode()?;
    */
    // leaving the alternate screen and disabling raw mode
    // (2nd)
    shutdown()?;

    // unwrapping the result
    // (3rd)
    status?;

    Ok(())
}

// breaking up the main() function
// (1) functuinality to initialize the terminal
fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
  }
  
// (2) functuinality to clean up the terminal
fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
  }
  
// (3) functionality to render the application state
fn ui(app: &mut App, f: &mut Frame) {
    f.render_widget(Paragraph::new(
        format!("Counter: {}", app.counter)), f.size()
    );
}
  
// (4) functionality to processes user input and update app state 
fn update(app: &mut App) -> Result<()> {
    // update state based on user input
    
    // break from loop based on user input and/or state
    // checking for user input
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            // widows sends key event twice, for KeyEventKind::Press and KeyEventKind::Release
            // so we've to make sure that key.kind is KeyEventKind::Press only
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    // 'j' adds 1 to the counter
                    Char('j') => app.counter += 1,
                    // 'k' subtracts 1 to from counter
                    Char('k') => app.counter -= 1,
                    // 'q' breaks the app
                    // break here would be outside of loop
                    // therefore we need the should_quit flag
                    Char('q') => app.should_quit = true,
                    _ => {},
                }
            }
        }
    }

    Ok(())
}

// (5) functionality that contains the main loop
fn run() -> Result<()> {
    // get a new ratatui terminal
    // creating an instance of a terminal backend with crossterm
    let mut terminal = Terminal::new(
        CrosstermBackend::new(std::io::stderr())
    )?;

    // set the application state
    let mut app = App {counter: 0, should_quit: false};

    // the main loop controlling the app
    loop {
        // render
        terminal.draw(|f| {
            ui(&mut app, f);
        })?;

        // update
        update(&mut app)?;

        // exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}