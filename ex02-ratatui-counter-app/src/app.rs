use std::io;
use ratatui::Frame;

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
    pub fn run() {
        unimplemented!()
    }
    
    pub fn finish(&mut self) {
        self.running_state = RunningState::Finished;
    } 

    pub fn is_finished(&mut self) -> bool {
        self.running_state == RunningState::Finished
    }

    pub fn render_frame(&mut self, frame: &mut Frame) {
        unimplemented!()
    }

    pub fn update(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}