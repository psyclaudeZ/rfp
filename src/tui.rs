use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    widgets::{List, ListItem, ListState},
    DefaultTerminal, Frame,
};
use std::io::{self};

struct Selectables {
    items: Vec<String>,
    state: ListState,
}

impl Selectables {
    fn new(items: Vec<String>) -> Selectables {
        assert!(items.len() > 0);
        let mut s = ListState::default();
        s.select(Some(0));
        Selectables { items, state: s }
    }
}

pub fn run(candidates: &[String]) -> io::Result<()> {
    if candidates.len() == 0 {
        print!("No paths found!");
        return Ok(());
    }
    let terminal = ratatui::init();
    let mut selectables = Selectables::new(candidates.to_vec());
    let result = run_selection(terminal, &mut selectables);
    ratatui::restore();
    result
}

fn run_selection(mut terminal: DefaultTerminal, selectables: &mut Selectables) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, selectables))?;
        // TODO: error handling
        handle_keypress(selectables)?;
    }
}

fn handle_keypress(selectables: &mut Selectables) -> io::Result<()> {
    let Event::Key(key) = event::read()? else {
        return Ok(());
    };
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    match key.code {
        KeyCode::Char('j') => selectables.state.select_next(),
        KeyCode::Char('k') => selectables.state.select_previous(),
        // TODO: is this the Rustacean way?
        _ => return Err(io::Error::new(io::ErrorKind::Interrupted, "user quit")),
    }
    Ok(())
}

fn render(frame: &mut Frame, selectables: &mut Selectables) {
    let items: Vec<ListItem> = selectables
        .items
        .iter()
        .map(|cand| ListItem::new(cand.as_str()))
        .collect();
    let list = List::new(items).highlight_symbol(">");
    frame.render_stateful_widget(list, frame.area(), &mut selectables.state);
}
