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

struct TUIState {
    items: Vec<String>,
    cursor: ListState,
    selected: Vec<bool>,
    main_area_height: u16,
}

impl TUIState {
    fn new(items: Vec<String>) -> TUIState {
        assert!(!items.is_empty());
        let len = items.len();
        let mut s = ListState::default();
        s.select(Some(0));
        TUIState {
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
    let mut tui_state = TUIState::new(candidates);
    let result = run_selection(terminal, &mut tui_state);
    ratatui::restore();
    result
}

fn run_selection(
    mut terminal: DefaultTerminal,
    tui_state: &mut TUIState,
) -> io::Result<(Vec<String>, TUILoopEvent)> {
    loop {
        terminal.draw(|frame| render(frame, tui_state))?;
        // TODO: error handling
        match handle_keypress(tui_state)? {
            TUILoopEvent::Continue => {}
            TUILoopEvent::Quit => break Ok((vec![], TUILoopEvent::Quit)),
            TUILoopEvent::Submit => {
                break Ok((
                    tui_state
                        .selected
                        .iter()
                        .enumerate()
                        .filter(|(_, is_selected)| **is_selected)
                        .map(|(i, _)| tui_state.items[i].clone())
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

fn handle_keypress(tui_state: &mut TUIState) -> io::Result<TUILoopEvent> {
    let Event::Key(key) = event::read()? else {
        return Ok(TUILoopEvent::Continue);
    };
    if key.kind != KeyEventKind::Press {
        return Ok(TUILoopEvent::Continue);
    }

    match key.code {
        // down
        KeyCode::Char('j') => {
            if tui_state.cursor.selected().unwrap() == tui_state.items.len() - 1 {
                tui_state.cursor.select_first()
            } else {
                tui_state.cursor.select_next()
            }
        }
        // down by 1/2 page
        KeyCode::Char('d') => {
            if tui_state.cursor.selected().unwrap() == tui_state.items.len() - 1 {
                tui_state.cursor.select_first()
            } else {
                tui_state
                    .cursor
                    .scroll_down_by(tui_state.main_area_height / 2);
            }
        }
        // down by 1 page
        KeyCode::Char('f') => {
            if tui_state.cursor.selected().unwrap() == tui_state.items.len() - 1 {
                tui_state.cursor.select_first()
            } else {
                tui_state.cursor.scroll_down_by(tui_state.main_area_height);
            }
        }
        // up
        KeyCode::Char('k') => {
            if tui_state.cursor.selected().unwrap() == 0 {
                tui_state.cursor.select_last()
            } else {
                tui_state.cursor.select_previous()
            }
        }
        // up by 1/2 page
        KeyCode::Char('u') => {
            if tui_state.cursor.selected().unwrap() == 0 {
                tui_state.cursor.select_last()
            } else {
                tui_state
                    .cursor
                    .scroll_up_by(tui_state.main_area_height / 2);
            }
        }
        // up by 1 page
        KeyCode::Char('b') => {
            if tui_state.cursor.selected().unwrap() == 0 {
                tui_state.cursor.select_last()
            } else {
                tui_state.cursor.scroll_up_by(tui_state.main_area_height);
            }
        }
        KeyCode::Char('g') => tui_state.cursor.select_first(),
        KeyCode::Char('G') => tui_state.cursor.select_last(),
        KeyCode::Char(' ') => {
            let idx = tui_state
                .cursor
                .selected()
                .expect("There should always be one item selected.");
            tui_state.selected[idx] = !tui_state.selected[idx];
        }
        KeyCode::Char('a') => {
            if tui_state.selected.iter().all(|&s| s) {
                tui_state.selected.fill(false)
            } else {
                tui_state.selected.fill(true)
            }
        }
        KeyCode::Char('q') => return Ok(TUILoopEvent::Quit),
        KeyCode::Esc => return Ok(TUILoopEvent::Quit),
        KeyCode::Enter => return Ok(TUILoopEvent::Submit),
        _ => return Ok(TUILoopEvent::Continue),
    }
    Ok(TUILoopEvent::Continue)
}

fn render(frame: &mut Frame, tui_state: &mut TUIState) {
    let items: Vec<ListItem> = tui_state
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            ListItem::new(item.as_str()).style(if tui_state.selected[i] {
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
    frame.render_stateful_widget(list, main_area, &mut tui_state.cursor);
    tui_state.main_area_height = main_area.height;
    frame.render_widget(
        Block::bordered().title_bottom(
            Line::from(format!(
                "{}/{}",
                tui_state.cursor.selected().unwrap() + 1,
                tui_state.items.len(),
            ))
            .right_aligned(),
        ),
        main_area,
    );
    frame.render_widget(Block::bordered(), sub_area);
}
