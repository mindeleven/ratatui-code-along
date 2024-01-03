/// coding along with the tutorials at https://ratatui.rs/
/// Example 02, Counter App @ https://ratatui.rs/tutorials/counter-app/
/// 
/// Create a TUI application
/// the goal of this demonstration is to build a simple counter application
/// the counter should be increment or decrement when a key is pressed
/// (j for increment and k for decrement)
/// 
/// importing the tui module
mod tui;

/// importing the necessary components from crossterm
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    // execute,
    // terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
/// importing the necessary components from ratatui
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::Paragraph
};

/// defining custom types and aliases
type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

/// defining an App struct to encapsulate the application state
/// derives Default trait to have reasonable defaults
/// App::default() will create an App with counter = 0 and running_state = RunningState::Running
#[derive(Debug, Default)]
struct App {
    // current state of the counter
    // an i64 for now, u8 later on according to new tutorial
    counter: i64,
    // flag that indicates whether app should exit main loop
    // gets replaced with running state, so remove later
    should_quit:  bool,
    // state of the app
    running_state: RunningState,
}

// representing the state of the application with an enum
#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Finished
}


fn main() -> Result<()> {
    // initialize the terminal
    // startup()?;
    tui::init()?;

    // (1st) we get the result of run() (the status)
    // then (2nd) we call shutdown() 
    // and after that (3rd) we unwrap() the result
    // be keeping this order, we ensure
    // that shutdown() is always called before the program exits

    // (1st)
    let status = run();

    // leaving the alternate screen and disabling raw mode
    // (2nd)
    // shutdown()?;
    tui::restore()?;

    // unwrapping the result
    // (3rd)
    status?;

    Ok(())
}

/* 
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
*/

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
    let mut app = App {counter: 0, should_quit: false, running_state: RunningState::Running};

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