use super::{
    colors::HIGHLIGHTED_COLOR,
    string_list::{self, StringList},
    SECONDARY_COLOR, TEXT_COLOR,
};
use crate::app::{self, App, Db};
use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table, TableState, Tabs, Widget,
        Wrap,
    },
    Frame,
};
use sqlformat::{format, FormatOptions, Indent, QueryParams};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Default, Debug, Display, EnumIter)]
pub enum SelectedTableTab {
    #[default]
    #[strum(to_string = "Browse")]
    Browse,
    #[strum(to_string = "Schema")]
    Schema,
}

impl SelectedTableTab {
    pub fn next(&self) -> SelectedTableTab {
        SelectedTableTab::iter()
            .nth(
                (*self as usize)
                    .saturating_add(1)
                    .clamp(0, SelectedTableTab::iter().len()),
            )
            .unwrap_or(SelectedTableTab::Schema)
    }

    pub fn previous(&self) -> SelectedTableTab {
        SelectedTableTab::iter()
            .nth(
                (*self as usize)
                    .saturating_sub(1)
                    .clamp(0, SelectedTableTab::iter().len()),
            )
            .unwrap_or(SelectedTableTab::Schema)
    }
}

impl Widget for SelectedTableTab {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Tabs::new(
            SelectedTableTab::iter().map(|x| Line::from(x.to_string()).fg(SECONDARY_COLOR).bold()),
        )
        .divider(symbols::DOT)
        .highlight_style(
            Style::default()
                .underlined()
                .underline_color(HIGHLIGHTED_COLOR),
        )
        .padding(" ", " ")
        .select(self as usize)
        .block(Block::default().borders(Borders::LEFT))
        .render(area, buf);
    }
}

#[derive(Clone, Copy, Default, Debug, Display, EnumIter)]
pub enum NavigationTab {
    #[default]
    #[strum(to_string = "Tables")]
    Tables,
    #[strum(to_string = "Views")]
    Views,
}

impl NavigationTab {
    pub fn next(&self) -> NavigationTab {
        NavigationTab::iter()
            .nth(
                (*self as usize)
                    .saturating_add(1)
                    .clamp(0, NavigationTab::iter().len()),
            )
            .unwrap_or(NavigationTab::Tables)
    }

    pub fn previous(&self) -> NavigationTab {
        NavigationTab::iter()
            .nth(
                (*self as usize)
                    .saturating_sub(1)
                    .clamp(0, NavigationTab::iter().len()),
            )
            .unwrap_or(NavigationTab::Tables)
    }
}

impl Widget for NavigationTab {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        Tabs::new(
            NavigationTab::iter().map(|x| Line::from(x.to_string()).fg(SECONDARY_COLOR).bold()),
        )
        .divider(symbols::DOT)
        .highlight_style(
            Style::default()
                .underlined()
                .underline_color(HIGHLIGHTED_COLOR),
        )
        .padding(" ", " ")
        .select(self as usize)
        .block(Block::default())
        .render(area, buf);
    }
}

pub struct TableView {
    pub tables_list: StringList,
    pub view_list: StringList,
    pub selected_table_tab: SelectedTableTab,
    pub table_nav_tab: NavigationTab,
    pub data: (Vec<String>, Vec<Vec<String>>),
    pub table_state: TableState,
    table_scroll_height: u16,
    clipboard: Option<Clipboard>,
    page_size: usize,
    offset: usize,
    pub total_rows: Option<usize>,
}

impl Default for TableView {
    fn default() -> Self {
        Self {
            tables_list: StringList::default(),
            view_list: StringList::default(),
            selected_table_tab: SelectedTableTab::default(),
            table_nav_tab: NavigationTab::default(),
            data: (Vec::default(), Vec::default()),
            table_state: TableState::default(),
            table_scroll_height: 0,
            clipboard: Clipboard::new().ok(),
            page_size: 50,
            offset: 0,
            total_rows: None,
        }
    }
}

impl TableView {
    pub fn load_nav(&mut self, db: &Db) {
        self.tables_list.load_items(
            db.tables
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>()
                .clone(),
        );
        self.view_list.load_items(
            db.views
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<_>>()
                .clone(),
        );
    }

    pub fn draw(&mut self, frame: &mut Frame, db: &Db) {
        let [l, r] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(4)])
            .margin(2)
            .areas(frame.area());
        let nav_tab_inner = Layout::vertical([Constraint::Fill(1)])
            .vertical_margin(2)
            .split(l);

        //Tabs
        frame.render_widget(self.table_nav_tab, l);
        frame.render_widget(self.selected_table_tab, r);

        // Nav Lists
        self.draw_nav_lists(frame, nav_tab_inner[0]);

        // Table Body
        if let Some(table) = self.get_selected_table(db) {
            self.draw_body(frame, table, r);
        }
    }

    fn get_selected_table<'a>(&self, db: &'a Db) -> Option<&'a app::Table> {
        match self.table_nav_tab {
            NavigationTab::Tables => {
                if let Some(table_name) = self.tables_list.get_selected() {
                    if let Some(table) = db.tables.iter().find(|x| x.name == table_name) {
                        return Some(table);
                    }
                }
                None
            }
            NavigationTab::Views => {
                if let Some(table_name) = self.view_list.get_selected() {
                    if let Some(table) = db.views.iter().find(|x| x.name == table_name) {
                        return Some(table);
                    }
                }
                None
            }
        }
    }

    fn draw_nav_lists(&mut self, frame: &mut Frame, area: Rect) {
        match self.table_nav_tab {
            NavigationTab::Tables => {
                frame.render_stateful_widget(
                    string_list::to_widget(&self.tables_list.items),
                    area,
                    &mut self.tables_list.list_state,
                );
            }
            NavigationTab::Views => {
                frame.render_stateful_widget(
                    string_list::to_widget(&self.view_list.items),
                    area,
                    &mut self.tables_list.list_state,
                );
            }
        }
    }

    fn draw_body(&mut self, frame: &mut Frame, table: &app::Table, r: Rect) {
        let margin = 2;
        match self.selected_table_tab {
            SelectedTableTab::Schema => {
                let lay = Layout::vertical([Constraint::Fill(1)])
                    .margin(margin)
                    .split(r);
                /*
                let p = Paragraph::new(table.sql.trim())
                    .wrap(Wrap { trim: true })
                    .fg(TEXT_COLOR);
                frame.render_widget(p, lay[0]);
                */

                let raw_sql = table.sql.trim();

                // build your options
                let opts = FormatOptions {
                    indent: Indent::Spaces(4), // 4‑space indent
                    uppercase: false,          // keep keywords lower‑case
                    lines_between_queries: 1,  // default
                };

                // call the formatter (no Dialect parameter)
                let formatted = format(raw_sql, &QueryParams::None, opts);

                // then render `formatted` instead of `raw`
                let p = Paragraph::new(formatted)
                    .wrap(Wrap { trim: false })
                    .fg(TEXT_COLOR);
                frame.render_widget(p, lay[0]);
            }
            SelectedTableTab::Browse => {
                // new: table, then preview, then footer for page‑info
                let lay = Layout::vertical([
                    Constraint::Fill(1),   // table takes all leftover space
                    Constraint::Length(3), // preview box height
                    Constraint::Length(1), // exactly one row for page info
                ])
                .margin(margin)
                .split(r);

                // draw the data table
                self.draw_table(frame, lay[0], table.name.as_str());

                // draw the preview inside a bordered box
                self.draw_preview(frame, lay[1]);

                // draw the page‑info *below* the preview
                if let Some(total) = self.total_rows {
                    let page = (self.offset / self.page_size) + 1;
                    let last_page = ((total + self.page_size - 1) / self.page_size).max(1);
                    let start = self.offset + 1;
                    let end = (self.offset + self.page_size).min(total);
                    let info = format!(
                        "displaying records {start}-{end} of {total}  (page {page}/{last_page})"
                    );

                    use ratatui::layout::Alignment;
                    use ratatui::widgets::Paragraph;

                    // render it centered/right‑aligned in that one‑row footer
                    frame.render_widget(
                        Paragraph::new(info)
                            .alignment(Alignment::Right)
                            .wrap(ratatui::widgets::Wrap { trim: true }),
                        lay[2],
                    );
                }
            }
        }
    }

    fn draw_preview(&mut self, frame: &mut Frame, table_inner: Rect) {
        if let Some((x, y)) = self.table_state.selected_cell() {
            if let Some(row) = self.data.1.get(x) {
                if let Some(val) = row.get(y) {
                    let p =
                        Paragraph::new(val.as_str())
                            .wrap(Wrap { trim: true })
                            .fg(TEXT_COLOR)
                            .block(Block::bordered().border_type(BorderType::Rounded).title(
                                " Preview ".fg(SECONDARY_COLOR).bold().into_centered_line(),
                            ));
                    frame.render_widget(p, table_inner);
                }
            }
        }
    }

    pub fn draw_table(&mut self, frame: &mut Frame, area: Rect, name: &str) {
        let (table_colums, data) = &self.data;
        let mut widths: Vec<usize> = Vec::default();
        let rows: Vec<Row> = data
            .iter()
            .enumerate()
            .map(|(i, text)| map_to_row(&mut widths, i, text))
            .collect();
        let headers: Vec<Cell> = table_colums
            .iter()
            .enumerate()
            .map(|(i, text)| map_to_cell_calc_width(&mut widths, i, text))
            .collect();
        let widths: Vec<Constraint> = widths
            .iter()
            .map(|x| Constraint::Length(*x as u16))
            .collect();
        let table = Table::new(rows, widths)
            .column_spacing(2)
            .style(Style::new().fg(TEXT_COLOR))
            .header(Row::new(headers).underlined().bold())
            .block(
                Block::bordered()
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded)
                    .title(name.fg(SECONDARY_COLOR).bold().into_centered_line()),
            )
            .cell_highlight_style(Style::new().reversed());
        frame.render_stateful_widget(table, area, &mut self.table_state);
        self.table_scroll_height = area.height / 2;
    }

    pub fn handle_input(
        &mut self,
        key: &KeyEvent,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(db) = &app.current_db {
            if key.code == KeyCode::Char('h') {
                self.table_state.scroll_left_by(1);
                return Ok(());
            } else if key.code == KeyCode::Char('u') {
                self.table_state.scroll_up_by(self.table_scroll_height);
                return Ok(());
            } else if key.code == KeyCode::Char('d') {
                self.table_state.scroll_down_by(self.table_scroll_height);
                return Ok(());
            } else if key.code == KeyCode::Char('l') {
                self.table_state.scroll_right_by(1);
                return Ok(());
            } else if key.code == KeyCode::Char('k') {
                self.table_state.scroll_up_by(1);
                return Ok(());
            } else if key.code == KeyCode::Char('j') {
                self.table_state.scroll_down_by(1);
                return Ok(());
            } else if key.code == KeyCode::Char('e') {
                self.table_nav_tab = self.table_nav_tab.next();
            } else if key.code == KeyCode::Char('q') {
                self.table_nav_tab = self.table_nav_tab.previous();
            } else if key.code == KeyCode::Char('L') {
                self.selected_table_tab = self.selected_table_tab.next();
            } else if key.code == KeyCode::Char('H') {
                self.selected_table_tab = self.selected_table_tab.previous();
            } else if key.code == KeyCode::Char('K') {
                self.tables_list.list_state.select_previous();
            } else if key.code == KeyCode::Char('J') {
                self.tables_list.list_state.select_next();
            } else if key.code == KeyCode::Char('y') {
                self.yank_cell()?;
                return Ok(());
            } else if key.code == KeyCode::Char('n') {
                // next page: only if there’s more data
                if let Some(total) = self.total_rows {
                    let next_off = self.offset + self.page_size;
                    if next_off < total {
                        self.offset = next_off;
                    }
                }
            } else if key.code == KeyCode::Char('p') {
                // prev page: never go below zero
                if self.offset >= self.page_size {
                    self.offset -= self.page_size;
                } else {
                    self.offset = 0;
                }
            }
            self.load_table_data(app, db)?;
        }
        Ok(())
    }

    fn yank_cell(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(if let Some((x, y)) = self.table_state.selected_cell() {
            if let Some(row) = self.data.1.get(x) {
                if let Some(val) = row.get(y) {
                    if let Some(cb) = &mut self.clipboard {
                        if let Err(e) = cb.set_text(val) {
                            eprintln!("Clipboard warning: {}", e);
                        }
                    }
                    return Ok(());
                }
            }
        })
    }

    fn load_table_data(&mut self, app: &App, db: &Db) -> Result<(), Box<dyn std::error::Error>> {
        // always reset the cursor to top-left of the page
        self.table_state.select_cell(Some((0, 0)));

        if self.selected_table_tab as usize == SelectedTableTab::Browse as usize {
            // pick the currently selected Table or View
            let maybe_table = match self.table_nav_tab {
                NavigationTab::Tables => db
                    .tables
                    .iter()
                    .find(|t| t.name == self.tables_list.get_selected().unwrap_or("")),
                NavigationTab::Views => db
                    .views
                    .iter()
                    .find(|v| v.name == self.view_list.get_selected().unwrap_or("")),
            };

            if let Some(table) = maybe_table {
                // load just one page of data
                let (cols, rows) = app.select(table, self.page_size, self.offset)?;
                self.data = (cols, rows);

                // now fetch and remember the grand total
                self.total_rows = Some(app.prepare_total_rows(table)?);
            }
        }
        Ok(())
    }
}

fn map_to_row<'a>(widths: &mut Vec<usize>, index: usize, row_data: &[String]) -> Row<'a> {
    let mut style = Style::new();
    if index % 2 != 0 {
        style = style.bg(Color::Black);
    }
    Row::new(
        row_data
            .iter()
            .enumerate()
            .map(|(i, x)| map_to_cell_calc_width(widths, i, x))
            .collect::<Vec<Cell>>(),
    )
    .style(style)
}

fn map_to_cell_calc_width<'a>(widths: &mut Vec<usize>, index: usize, text: &String) -> Cell<'a> {
    let value = Text::from(text.to_string());
    if let Some(w) = widths.get_mut(index) {
        *w = (*w).max(value.width())
    } else {
        widths.push(value.width());
    }
    Cell::from(value)
}
