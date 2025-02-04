use crate::ui::{PRIMARY_COLOR, SECONDARY_COLOR, TEXT_COLOR};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Clear, Row, Table},
    Frame,
};

const FILE_MENU_KEYS: [[&str; 2]; 3] = [["Up", "k"], ["Down", "j"], ["Select", "Enter"]];
const FILE_TITLE: &str = "File Menu Keybinds";

const MAIN_VIEW_TITLE: &str = "Main View Keybinds";
const TAB_BAR_KEYS: [[&str; 2]; 2] = [
    ["Switch Tables - Views", "q - e"],
    ["Switch View Schema - Browse Data", "h - l"],
];

const NAV_LIST_TITLE: &str = "Navigation List Keybinds (Left side)";
const NAV_LIST_KEYS: [[&str; 2]; 2] = [["Up", "k"], ["Down", "j"]];

const TABLE_VIEW_TITLE: &str = "Table View Keybinds";
const TABLE_KEYS: [[&str; 2]; 6] = [
    ["Page Up Half", "u"],
    ["Page Down Half", "d"],
    ["Move Cell Up", "shift + k"],
    ["Move Cell Down", "shift + j"],
    ["Move Cell Left", "shift + h"],
    ["Move Cell Right", "shift + l"],
];

pub fn draw_help_window(frame: &mut Frame, lay: Rect) {
    let background = Block::bordered()
        .title(Line::from(" HELP ").fg(SECONDARY_COLOR).bold().centered())
        .fg(PRIMARY_COLOR)
        .border_type(BorderType::Rounded);
    frame.render_widget(Clear, lay);
    frame.render_widget(background, lay);

    let l = Layout::vertical(Constraint::from_lengths([5, 4, 4, 10]))
        .margin(2)
        .split(lay);

    let file_menu_table = Table::new(
        FILE_MENU_KEYS.map(|x| Row::new(x)),
        Constraint::from_mins([0, 0]),
    )
    .fg(TEXT_COLOR)
    .block(Block::default().title(FILE_TITLE.underlined()));
    frame.render_widget(file_menu_table, l[0]);

    let main_view_table = Table::new(
        TAB_BAR_KEYS.map(|x| Row::new(x)),
        Constraint::from_mins([0, 0]),
    )
    .fg(TEXT_COLOR)
    .block(Block::default().title(MAIN_VIEW_TITLE.underlined()));
    frame.render_widget(main_view_table, l[1]);

    let nav_list_table = Table::new(
        NAV_LIST_KEYS.map(|x| Row::new(x)),
        Constraint::from_mins([0, 0]),
    )
    .fg(TEXT_COLOR)
    .block(Block::default().title(NAV_LIST_TITLE.underlined()));
    frame.render_widget(nav_list_table, l[2]);

    let table_view_table = Table::new(
        TABLE_KEYS.map(|x| Row::new(x)),
        Constraint::from_mins([0, 0]),
    )
    .fg(TEXT_COLOR)
    .block(Block::default().title(TABLE_VIEW_TITLE.underlined()));
    frame.render_widget(table_view_table, l[3]);
}
