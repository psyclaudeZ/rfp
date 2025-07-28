use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Flex, Layout, Rect},
    prelude::Constraint,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph, Wrap},
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
    cursor: ListState,
    is_showing_help: bool,
    items: Vec<String>,
    main_area_height: u16,
    selected: Vec<bool>,
}

// h, l -
const HELP_MESSAGE_ENTRIES: &[(&str, &str)] = &[
    ("q", "Exit"),
    ("h", "Toggle help"),
    ("", ""),
    ("space", "Toggle selection"),
    ("enter", "Open selected files"),
    ("a", "Select all/none"),
    ("", ""),
    ("k/↑", "Move up"),
    ("j/↓", "Move down"),
    ("u", "Half page up"),
    ("d", "Half page down"),
    ("b/PgUp", "Full page up"),
    ("f/PgDn", "Full page down"),
    ("g/Home", "Go to top"),
    ("G/End", "Go to bottom"),
];

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
            is_showing_help: false,
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
        KeyCode::Char('j') | KeyCode::Down => {
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
        KeyCode::Char('f') | KeyCode::PageDown => {
            if tui_state.cursor.selected().unwrap() == tui_state.items.len() - 1 {
                tui_state.cursor.select_first()
            } else {
                tui_state.cursor.scroll_down_by(tui_state.main_area_height);
            }
        }
        // up
        KeyCode::Char('k') | KeyCode::Up => {
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
        KeyCode::Char('b') | KeyCode::PageUp => {
            if tui_state.cursor.selected().unwrap() == 0 {
                tui_state.cursor.select_last()
            } else {
                tui_state.cursor.scroll_up_by(tui_state.main_area_height);
            }
        }
        // top of the list
        KeyCode::Char('g') | KeyCode::Home => tui_state.cursor.select_first(),
        // bottom of the list
        KeyCode::Char('G') | KeyCode::End => tui_state.cursor.select_last(),
        KeyCode::Char('h') => tui_state.is_showing_help = !tui_state.is_showing_help,
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
        .highlight_symbol(">>")
        .block(Block::bordered());
    let [main_area, sub_area] =
        Layout::vertical([Constraint::Percentage(98), Constraint::Percentage(2)])
            .areas(frame.area());
    frame.render_stateful_widget(list, main_area, &mut tui_state.cursor);
    tui_state.main_area_height = main_area.height;
    frame.render_widget(
        Block::bordered()
            .title_bottom(
                Line::from(format!(
                    " {}/{} ",
                    tui_state.cursor.selected().unwrap() + 1,
                    tui_state.items.len(),
                ))
                .left_aligned(),
            )
            .title_bottom(Line::from(" h for help ").right_aligned()),
        main_area,
    );
    frame.render_widget(Block::default(), sub_area);
    if tui_state.is_showing_help {
        render_help_message(frame);
    }
}

fn render_help_message(frame: &mut Frame) {
    // Setup
    let popup_block = Block::bordered().title_top(Line::from("Help").centered());
    let popup_area = popup_area(frame.area(), 40, 80);
    let content_area = popup_block.inner(popup_area);

    // This clears out the background, DO NOT REMOVE and core rendering should happen AFTER these.
    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);

    let [_margin, text_area] = Layout::vertical([
        Constraint::Length(1), // Top margin (1 line)
        Constraint::Min(0),
    ])
    .areas(content_area);

    let rows: [Rect; HELP_MESSAGE_ENTRIES.len()] = Layout::vertical(vec![
        Constraint::Length(1);
        // Constraint::Ratio(1, HELP_MESSAGE_ENTRIES.len() as u32);
        HELP_MESSAGE_ENTRIES.len()
    ])
    .areas(text_area);
    for (i, &(key, desc)) in HELP_MESSAGE_ENTRIES.iter().enumerate() {
        let columns: [Rect; 2] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(rows[i]);
        // Render content in each cell
        frame.render_widget(
            Paragraph::new(key).wrap(Wrap { trim: true }).centered(),
            columns[0],
        );
        frame.render_widget(
            Paragraph::new(desc)
                .wrap(Wrap { trim: true })
                .left_aligned(),
            columns[1],
        );
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
