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

    fn step(&mut self, is_increment: bool) {
        let length = self.items.len();
        let i = match self.state.selected() {
            Some(i) => {
                // Hmm can I use tenerary
                if is_increment {
                    if i + 1 >= length {
                        0
                    } else {
                        i + 1
                    }
                } else {
                    if i == 0 {
                        length - 1
                    } else {
                        i - 1
                    }
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn next(&mut self) {
        self.step(true);
    }

    fn prev(&mut self) {
        self.step(false);
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
        handle_events(selectables)?;
    }
}

fn handle_events(selectables: &mut Selectables) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('j') => selectables.next(),
                KeyCode::Char('k') => selectables.prev(),
                // TODO: is this the Rustacean way?
                _ => return Err(io::Error::new(io::ErrorKind::Interrupted, "user quit")),
            }
        }
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
