use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
    DefaultTerminal, Frame,
};
use std::io::{self};

enum KeyPressAction {
    Continue,
    Quit,
    Submit,
}

struct Selectables {
    items: Vec<String>,
    cursor: ListState,
    selected: Vec<bool>,
}

impl Selectables {
    fn new(items: Vec<String>) -> Selectables {
        assert!(!items.is_empty());
        let len = items.len();
        let mut s = ListState::default();
        s.select(Some(0));
        Selectables {
            items,
            cursor: s,
            selected: vec![false; len],
        }
    }
}

pub fn run(candidates: Vec<String>) -> io::Result<()> {
    if candidates.is_empty() {
        print!("No paths found!");
        return Ok(());
    }
    let terminal = ratatui::init();
    let mut selectables = Selectables::new(candidates);
    let result = run_selection(terminal, &mut selectables);
    ratatui::restore();
    let selected = result?;
    if selected.is_empty() {
        println!("Nothing selected.");
    } else {
        println!("Selected: {:?}", selected);
    }
    Ok(())
}

fn run_selection(
    mut terminal: DefaultTerminal,
    selectables: &mut Selectables,
) -> io::Result<Vec<String>> {
    loop {
        terminal.draw(|frame| render(frame, selectables))?;
        // TODO: error handling
        match handle_keypress(selectables)? {
            KeyPressAction::Continue => {}
            KeyPressAction::Quit => break Ok(vec![]),
            KeyPressAction::Submit => {
                break Ok(selectables
                    .selected
                    .iter()
                    .enumerate()
                    .filter(|(_, is_selected)| **is_selected)
                    .map(|(i, _)| selectables.items[i].clone())
                    .collect());
            }
        }
    }
}

fn handle_keypress(selectables: &mut Selectables) -> io::Result<KeyPressAction> {
    let Event::Key(key) = event::read()? else {
        return Ok(KeyPressAction::Continue);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(KeyPressAction::Continue);
    }

    match key.code {
        KeyCode::Char('j') => selectables.cursor.select_next(),
        KeyCode::Char('k') => selectables.cursor.select_previous(),
        KeyCode::Char(' ') => {
            let idx = selectables
                .cursor
                .selected()
                .expect("There should always be one item selected.");
            selectables.selected[idx] = !selectables.selected[idx];
        }
        KeyCode::Char('a') => {
            if selectables.selected.iter().all(|&s| s) {
                selectables.selected.fill(false)
            } else {
                selectables.selected.fill(true)
            }
        }
        KeyCode::Enter => return Ok(KeyPressAction::Submit),
        _ => return Ok(KeyPressAction::Quit),
    }
    Ok(KeyPressAction::Continue)
}

fn render(frame: &mut Frame, selectables: &mut Selectables) {
    let items: Vec<ListItem> = selectables
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            ListItem::new(item.as_str()).style(if selectables.selected[i] {
                Style::default().bg(Color::LightBlue)
            } else {
                Style::default()
            })
        })
        .collect();
    let list = List::new(items).highlight_symbol(">");
    frame.render_stateful_widget(list, frame.area(), &mut selectables.cursor);
}
