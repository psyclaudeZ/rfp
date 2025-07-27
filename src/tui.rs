use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Layout,
    prelude::Constraint,
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState},
    DefaultTerminal, Frame,
};
use std::io::{self};

#[derive(PartialEq)]
pub enum TUILoopEvent {
    EarlyReturn,
    Continue,
    Quit,
    Submit,
}

struct Selectables {
    items: Vec<String>,
    cursor: ListState,
    selected: Vec<bool>,
    main_area_height: u16,
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
            main_area_height: 0,
        }
    }
}

pub fn run(candidates: Vec<String>) -> io::Result<(Vec<String>, TUILoopEvent)> {
    if candidates.is_empty() {
        return Ok((vec![], TUILoopEvent::EarlyReturn));
    }
    let terminal = ratatui::init();
    let mut selectables = Selectables::new(candidates);
    let result = run_selection(terminal, &mut selectables);
    ratatui::restore();
    result
}

fn run_selection(
    mut terminal: DefaultTerminal,
    selectables: &mut Selectables,
) -> io::Result<(Vec<String>, TUILoopEvent)> {
    loop {
        terminal.draw(|frame| render(frame, selectables))?;
        // TODO: error handling
        match handle_keypress(selectables)? {
            TUILoopEvent::Continue => {}
            TUILoopEvent::Quit => break Ok((vec![], TUILoopEvent::Quit)),
            TUILoopEvent::Submit => {
                break Ok((
                    selectables
                        .selected
                        .iter()
                        .enumerate()
                        .filter(|(_, is_selected)| **is_selected)
                        .map(|(i, _)| selectables.items[i].clone())
                        .collect(),
                    TUILoopEvent::Submit,
                ));
            }
            TUILoopEvent::EarlyReturn => {
                panic!("Key press yields an invalid event!");
            }
        }
    }
}

fn handle_keypress(selectables: &mut Selectables) -> io::Result<TUILoopEvent> {
    let Event::Key(key) = event::read()? else {
        return Ok(TUILoopEvent::Continue);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(TUILoopEvent::Continue);
    }

    match key.code {
        // down
        KeyCode::Char('j') => {
            if selectables.cursor.selected().unwrap() == selectables.items.len() - 1 {
                selectables.cursor.select_first()
            } else {
                selectables.cursor.select_next()
            }
        }
        // down by 1/2 page
        KeyCode::Char('d') => {
            if selectables.cursor.selected().unwrap() == selectables.items.len() - 1 {
                selectables.cursor.select_first()
            } else {
                selectables
                    .cursor
                    .scroll_down_by(selectables.main_area_height / 2);
            }
        }
        // down by 1 page
        KeyCode::Char('f') => {
            if selectables.cursor.selected().unwrap() == selectables.items.len() - 1 {
                selectables.cursor.select_first()
            } else {
                selectables
                    .cursor
                    .scroll_down_by(selectables.main_area_height);
            }
        }
        // up
        KeyCode::Char('k') => {
            if selectables.cursor.selected().unwrap() == 0 {
                selectables.cursor.select_last()
            } else {
                selectables.cursor.select_previous()
            }
        }
        // up by 1/2 page
        KeyCode::Char('u') => {
            if selectables.cursor.selected().unwrap() == 0 {
                selectables.cursor.select_last()
            } else {
                selectables
                    .cursor
                    .scroll_up_by(selectables.main_area_height / 2);
            }
        }
        // up by 1 page
        KeyCode::Char('b') => {
            if selectables.cursor.selected().unwrap() == 0 {
                selectables.cursor.select_last()
            } else {
                selectables
                    .cursor
                    .scroll_up_by(selectables.main_area_height);
            }
        }
        KeyCode::Char('g') => selectables.cursor.select_first(),
        KeyCode::Char('G') => selectables.cursor.select_last(),
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
        KeyCode::Char('q') => return Ok(TUILoopEvent::Quit),
        KeyCode::Esc => return Ok(TUILoopEvent::Quit),
        KeyCode::Enter => return Ok(TUILoopEvent::Submit),
        _ => return Ok(TUILoopEvent::Continue),
    }
    Ok(TUILoopEvent::Continue)
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
    let list = List::new(items)
        .highlight_symbol(">")
        .block(Block::bordered());
    let [main_area, sub_area] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
            .areas(frame.area());
    frame.render_stateful_widget(list, main_area, &mut selectables.cursor);
    selectables.main_area_height = main_area.height;
    frame.render_widget(
        Block::bordered().title_bottom(
            Line::from(format!(
                "{}/{} - {}",
                selectables.cursor.selected().unwrap() + 1,
                selectables.items.len(),
                main_area.height,
            ))
            .right_aligned(),
        ),
        main_area,
    );
    frame.render_widget(Block::bordered(), sub_area);
}
