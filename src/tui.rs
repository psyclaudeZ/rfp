use crossterm::event::{self, Event};
use ratatui::{
    widgets::{List, ListItem},
    DefaultTerminal, Frame,
};
use std::io::{self};

pub fn run(candidates: &[String]) -> io::Result<()> {
    let terminal = ratatui::init();
    run_selection(terminal, candidates)?;
    ratatui::restore();
    Ok(())
}

fn run_selection(mut terminal: DefaultTerminal, candidates: &[String]) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, candidates))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
fn render(frame: &mut Frame, candidates: &[String]) {
    if candidates.len() == 0 {
        frame.render_widget("No file paths are found!", frame.area())
    }
    let items: Vec<ListItem> = candidates
        .iter()
        .map(|cand| ListItem::new(cand.as_str()))
        .collect();
    frame.render_widget(List::new(items), frame.area());
}
