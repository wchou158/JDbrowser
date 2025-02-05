use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, List},
    Frame,
};

use super::{string_list::StringList, utils::center, HIGHLIGHT_STYLE, SECONDARY_COLOR};

pub fn draw(frame: &mut Frame, list: &mut StringList) {
    let title = Line::from(" Select Database ")
        .fg(SECONDARY_COLOR)
        .bold()
        .centered();
    let mut max_width = title.width();

    let text_items = map_to_text(&list.items);
    for txt in &text_items {
        max_width = max_width.max(txt.width());
    }

    let widget = List::new(text_items)
        .highlight_style(HIGHLIGHT_STYLE)
        .highlight_symbol(">")
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title),
        );
    let area = center(
        frame.area(),
        Constraint::Length((max_width as u16) + 15),
        Constraint::Percentage(50),
    );
    frame.render_stateful_widget(widget, area, &mut list.list_state);
}

fn map_to_text(list: &[String]) -> Vec<Text> {
    let text_items = list
        .iter()
        .map(|x| {
            let txt = Text::from(x.as_str()).fg(SECONDARY_COLOR).bold();
            txt
        })
        .collect::<Vec<Text>>();
    text_items
}
