use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use mdtohtml;
use std::io::stdout;

fn main() -> Result<()> {
    // Get the output.html from the .md provided
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();

    mdtohtml::load(file, true, true);
    open::that("./output.html").expect("");

    enable_raw_mode()?;
    println!("To exit: press `q`");
    println!("To hot-reload: press `r`");
    loop {
        // Check if a key was pressed, and then check what
        if let Event::Key(KeyEvent { code, .. }) = crossterm::event::read()? {
            match code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => open::that("./output.html").expect(""),
                // Handle other keys here
                _ => {}
            }
        }

        // Wait for a short period of time
        std::thread::sleep(std::time::Duration::from_millis(400));
    }

    disable_raw_mode()?;
    execute!(stdout(), crossterm::cursor::Show)?; // Exit

    Ok(())
}
