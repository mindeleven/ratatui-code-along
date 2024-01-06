use std::io;
use crossterm::event::{
    self,
    Event, 
    KeyCode,
    KeyEvent
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        *,
    },
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
        // while implements the main loop
        while !self.is_finished() {
            terminal.draw(|frame| {
                self.render_frame(frame);
            })?;
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
    
    // rendering the frame, source code from 
    // https://counter-tutorial-rewrite.ratatui.pages.dev/tutorials/counter-app/basic-app/
    // TODO: some research on styling widgets
    pub fn render_frame(&mut self, frame: &mut Frame) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(
            Line::from(vec![
                " Decrement ".into(),
                "<Left>".blue().bold(),
                " Increment ".into(),
                "<Right>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ])
        );
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            // here we get the counter
            self.counter.to_string().yellow(),
        ])]);

        frame.render_widget(
            Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(block),
            frame.size()
        );
    }
  
    // application only accepts key events via the standard iput
    // function updates the app's state based on user input
    pub fn update(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        };
        Ok(())
    }

    // helper functions for handling keyboard events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != event::KeyEventKind::Press {
            return;
        }
        match key_event.code {
            // 'j' adds 1 to the counter
            KeyCode::Right => self.increment(),
            // 'k' subtracts 1 to from counter
            KeyCode::Left => self.decrement(),
            // 'q' breaks the self.                        
            // break here would be outside of loop
            // therefore we need the should_quit flag
            KeyCode::Char('q') | KeyCode::Char('Q') => self.finish(),
            _ => {},
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }

}

/// testing the UI Output of render_frame with TestBackend
/// https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html
#[cfg(test)]
mod test {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn render_frame() {
        let mut app = App::default();
        let backend = TestBackend::new(50, 4);
        let mut terminal = Terminal::new(backend).expect("terminal");
        
        terminal
            .draw(|frame| app.render_frame(frame))
            .expect("draw");

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        terminal.backend().assert_buffer(&expected);
    }

}

