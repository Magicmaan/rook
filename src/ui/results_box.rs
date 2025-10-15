use crate::ui::util::IconMode;
use crate::{
    model,
    settings::{UIResultsSettings, UISearchSettings},
};
use crate::{settings::Settings, ui::util::number_to_icon};
use ratatui::{
    buffer::Buffer,
    crossterm::terminal::Clear,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget,
    },
};
use std::time::SystemTime;
#[derive(Clone, Default)]
pub struct ResultsBox {
    results: Vec<(u16, usize)>,
    data: crate::model::Data,
    index: usize,
    settings: UIResultsSettings,
}

impl ResultsBox {
    pub fn new(
        results: Vec<(u16, usize)>,
        data: &crate::model::Data,
        index: usize,
        settings: UIResultsSettings,
    ) -> Self {
        Self {
            results,
            data: data.clone(),
            index,
            settings,
        }
    }
}

impl StatefulWidget for ResultsBox {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = Settings::new().ui.theme.clone();
        let block = Block::bordered()
            .border_style(Style::default())
            .border_type(theme.get_border_type("results"))
            // .title("Search")
            .padding(Padding::new(1, 1, 1, 1));

        // block.render(area, buf);
        let inner_area = block.inner(area);

        let mut i = 1;
        let items =
            self.results
                .iter()
                .map(|(score, idx)| {
                    // space out sections to fit the width
                    let app = &self.data.applications[*idx];
                    let width = inner_area.width as usize;
                    let score_text = format!("{}", score);
                    // pad score to end i.e. "App Name       123"
                    let mut name_width = width.saturating_sub(score_text.len());

                    // get number icon
                    // mode configurable in settings
                    let number_icon = number_to_icon(i, self.settings.number_mode);

                    // if number icon, reduce padding for name
                    if self.settings.numbered {
                        name_width = name_width.saturating_sub(number_icon.len() + 1); // +1 for space
                    }
                    let padded_name = format!("{:<width$}", app.name, width = name_width);
                    // construct line
                    let line = Line::from(vec![
                        if self.settings.numbered {
                            Span::styled(
                                format!("{} ", number_icon),
                                Style::default().fg(theme
                                    .get_color("accent", Some(crate::ui::ui::UISection::Results))),
                            )
                        } else {
                            Span::raw("")
                        },
                        Span::styled(padded_name.clone(), Style::default().fg(Color::White)),
                        if self.settings.show_scores {
                            Span::styled(score_text.clone(), Style::default().fg(Color::DarkGray))
                        } else {
                            Span::raw("")
                        },
                    ]);
                    i += 1;
                    ListItem::new(line)
                })
                .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_symbol("")
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        // list.render(inner_area, buf);
        StatefulWidget::render(list, inner_area, buf, state);
        block.render(area, buf);
    }
}
