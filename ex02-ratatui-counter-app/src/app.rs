use std::io;
use crossterm::style::SetForegroundColor;

use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
};

use ratatui::{
    Frame, 
    Terminal, 
    backend::Backend,
    widgets::Paragraph
};

/// defining an App struct to encapsulate the application state
/// derives Default trait to have reasonable defaults
/// App::default() will create an App with counter = 0 and running_state = RunningState::Running
#[derive(Debug, Default)]
pub struct App {
    // current state of the counter
    // an i64 for now, u8 later on according to new tutorial
    pub counter: i64,
    // flag that indicates whether app should exit main loop
    // gets replaced with running state, so remove later
    pub should_quit:  bool,
    // state of the app
    pub running_state: RunningState,
}

// representing the state of the application with an enum
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Finished
}

// implementing the functionality of the main loop
impl App {
    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>
    ) -> io::Result<()> {
        while !self.is_finished() {
            terminal.draw(|frame| {
                self.render_frame(frame);
            });
            self.update()?;
        }
        Ok(())
    }
    
    pub fn finish(&mut self) {
        self.running_state = RunningState::Finished;
    } 

    pub fn is_finished(&mut self) -> bool {
        self.running_state == RunningState::Finished
    }

    pub fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(Paragraph::new(
            format!("Counter: {}", self.counter)), frame.size()
        );
    }

    pub fn update(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Key(key) = event::read()? {
                // widows sends key event twice, for KeyEventKind::Press and KeyEventKind::Release
                // so we've to make sure that key.kind is KeyEventKind::Press only
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        // 'j' adds 1 to the counter
                        Char('j') => self.counter += 1,
                        // 'k' subtracts 1 to from counter
                        Char('k') => self.counter -= 1,
                        // 'q' breaks the self.                        // break here would be outside of loop
                        // therefore we need the should_quit flag
                        Char('q') => self.finish(),
                        _ => {},
                    }
                }
            }
        }
    
        Ok(())
    }
}