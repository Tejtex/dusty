use std::time::Duration;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

mod app;

mod dirscreen;
mod filescreen;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(c) => {
                        println!("Pressed key: {}", c);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}