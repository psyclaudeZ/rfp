use crate::parser::MatchResult;
use crossterm::event::{self, Event};
use ratatui::{
    widgets::{List, ListItem},
    DefaultTerminal, Frame,
};
use std::io::{self};

pub fn run(matches: &Vec<MatchResult>) -> io::Result<()> {
    let terminal = ratatui::init();
    run_selection(terminal, &matches)?;
    ratatui::restore();
    Ok(())
}

fn run_selection(mut terminal: DefaultTerminal, matches: &Vec<MatchResult>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, matches))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
fn render(frame: &mut Frame, matches: &Vec<MatchResult>) {
    if matches.len() == 0 {
        frame.render_widget("No file paths are found!", frame.area())
    }
    let items: Vec<ListItem> = matches
        .iter()
        .map(|m| ListItem::new(m.path.clone()))
        .collect();
    frame.render_widget(List::new(items), frame.area());
}
