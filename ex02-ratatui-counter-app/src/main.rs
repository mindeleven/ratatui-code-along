/// coding along with the tutorials at https://ratatui.rs/
/// Example 02, Counter App @ https://ratatui.rs/tutorials/counter-app/
/// 
/// Create a TUI application
/// the goal of this demonstration is to build a simple counter application
/// the counter should be increment or decrement when a key is pressed
/// (j for increment and k for decrement)
/// 
/// importing the app module
mod app;
/// importing the tui module
mod tui;

fn main() -> std::io::Result<()> {
    // initialize the terminal
    let mut terminal = tui::init()?;

    // (1st) we get the result of run() (the status)
    // then (2nd) we call shutdown() 
    // and after that (3rd) we unwrap() the result
    // be keeping this order, we ensure
    // that shutdown() is always called before the program exits

    // (1st)
    let status = app::App::default().run(&mut terminal);

    // leaving the alternate screen and disabling raw mode
    // (2nd)
    tui::restore()?;

    // unwrapping the result
    // (3rd)
    status?;

    Ok(())
}