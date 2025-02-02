use crate::app::{load_files, App};
use colors::*;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType},
    Frame,
};
use std::io;
use string_list::StringList;
use talbe_view::TableView;

pub mod colors;
pub mod file_menu;
pub mod string_list;
pub mod talbe_view;

const APP_NAME: &str = " JDbrowser ";

#[derive(Debug)]
pub struct Ui {
    file_list: StringList,
    table_view: TableView,
}

impl Ui {
    pub fn new() -> io::Result<Self> {
        let mut file_list = StringList::default();
        file_list.load_items(load_files()?);
        Ok(Self {
            file_list,
            table_view: TableView::default(),
        })
    }

    pub fn ui(&mut self, frame: &mut Frame, app: &mut App) {
        let lay = Layout::horizontal([Constraint::Fill(1)])
            .margin(1)
            .split(frame.area());
        draw_outer_frame(frame, app, lay[0]);
        if let Some(db) = &app.current_db {
            self.table_view.draw(frame, db);
        } else {
            file_menu::draw(frame, &mut self.file_list);
        }
    }

    pub fn handle_input(
        &mut self,
        key: &KeyEvent,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(_db) = &app.current_db {
            self.table_view.handle_input(key, app)?;
        } else {
            self.handle_flist_input(key, app)?;
        }
        Ok(())
    }

    fn handle_flist_input(
        &mut self,
        key: &KeyEvent,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.file_list.handle_input(key);
        if key.code == KeyCode::Enter {
            if let Some(path) = self.file_list.get_selected() {
                app.load_db(path)?;
                if let Some(db) = &app.current_db {
                    self.table_view.load_nav(db);
                }
            }
        }
        Ok(())
    }
}

fn draw_outer_frame(frame: &mut Frame, app: &App, area: Rect) {
    let mut key_binds: Vec<Span> = vec![
        " Up ".into(),
        "<k>".fg(HIGHLIGHTED_COLOR),
        " Down ".into(),
        "<j>".fg(HIGHLIGHTED_COLOR),
    ];
    append_keybinds(app, &mut key_binds);
    frame.render_widget(new_outer_frame(app, key_binds), area);
}

fn append_keybinds(app: &App, key_binds: &mut Vec<Span>) {
    if let Some(_pat) = &app.current_db {
        let mut right_left_keys: Vec<Span> = vec![
            " Left ".into(),
            "<h>".fg(HIGHLIGHTED_COLOR),
            " Right ".into(),
            "<l>".fg(HIGHLIGHTED_COLOR),
        ];
        key_binds.append(&mut right_left_keys);
    } else {
        let mut enter_key: Vec<Span> = vec![" Select ".into(), "<Enter> ".fg(HIGHLIGHTED_COLOR)];
        key_binds.append(&mut enter_key);
    }
}

fn new_outer_frame<'a>(app: &App, key_binds: Vec<Span<'a>>) -> Block<'a> {
    let key_instruction = Line::from(key_binds).fg(SECONDARY_COLOR).bold().centered();
    let mut outer_frame = Block::bordered()
        .title(Line::from(APP_NAME).fg(SECONDARY_COLOR).bold().centered())
        .fg(PRIMARY_COLOR)
        .border_type(BorderType::Rounded);
    if let Some(_pat) = &app.current_db {
        outer_frame = outer_frame.title_bottom(key_instruction);
    } else {
        outer_frame = outer_frame.title_bottom(key_instruction);
    }
    outer_frame
}
