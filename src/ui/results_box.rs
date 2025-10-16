use crate::model::ui::UIState;

use crate::settings::settings::{Settings, UIResultsSettings};
use crate::ui::util::number_to_icon;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding, StatefulWidget, Widget},
};

use crate::model::ui::UISection;

#[derive(Clone, Default)]
pub struct ResultsBox {
    results: Vec<(u16, usize)>,
    data: crate::model::search::SearchData,
    settings: UIResultsSettings,
}

impl ResultsBox {
    pub fn new(
        results: Vec<(u16, usize)>,
        data: &crate::model::search::SearchData,
        settings: UIResultsSettings,
    ) -> Self {
        Self {
            results,
            data: data.clone(),
            settings,
        }
    }

    fn calculate_color_fade(&self, start_color: Color, position: usize, height: usize) -> Color {
        if let Color::Rgb(r, g, b) = start_color {
            let diff = height.saturating_sub(position);
            if diff < 5 {
                let base_brightness = 1.0;
                let brightness = 1.0
                    - maths_rs::lerp(base_brightness, 0.5, (diff as f32 / 2.5).min(1.0).max(0.25));
                Color::Rgb(
                    (brightness * r as f32) as u8,
                    (brightness * g as f32) as u8,
                    (brightness * b as f32) as u8,
                )
            } else {
                start_color
            }
        } else {
            // indexed / ANSI colours aren't supported for fine-grained fading, so just return the
            start_color
        }
    }
}

impl StatefulWidget for ResultsBox {
    type State = UIState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = Settings::new().ui.theme.clone();
        let block = Block::bordered()
            .border_style(Style::default())
            .border_type(theme.get_border_type("results"))
            // .title("Search")
            .padding(Padding::new(1, 1, 1, 1));

        // block.render(area, buf);
        let inner_area = block.inner(area);

        let available_height = inner_area.height as usize;

        let mut i = 1;
        let items = self
            .results
            .iter()
            .map(|(score, idx)| {
                // space out sections to fit the width
                let app = &self.data.applications[*idx];
                let width = inner_area.width as usize;
                let score_text = format!("{}", score);
                // pad score to end i.e. "App Name       123"
                let mut name_width = width.saturating_sub(score_text.len() - 1);

                // get number icon
                // mode configurable in settings
                let mut prepend_icon = number_to_icon(i, self.settings.number_mode);
                let executing_item = state.executing_item;
                if executing_item.is_some() && i == executing_item.unwrap() + 1 {
                    let tick = state.tick;
                    let remainder = tick % 4;
                    if remainder == 0 {
                        prepend_icon = "◜".to_string();
                    } else if remainder == 1 {
                        prepend_icon = "◝".to_string();
                    } else if remainder == 2 {
                        prepend_icon = "◞".to_string();
                    } else {
                        prepend_icon = "◟".to_string();
                    }
                    // prepend_icon = "XXXX".to_string();
                }
                // if number icon, reduce padding for name
                if self.settings.numbered && !prepend_icon.is_empty() {
                    name_width = name_width.saturating_sub(prepend_icon.len() + 1); // +1 for space
                }
                let padded_name = format!("{:<width$}", app.name, width = name_width);

                let mut text_color = theme.get_color("text", Some(UISection::Results));
                let mut muted_color = theme.get_color("text_muted", Some(UISection::Results));
                if self.settings.fade_color && available_height > 10 {
                    text_color = self.calculate_color_fade(
                        text_color,
                        i.saturating_sub(state.result_list_state.offset()),
                        available_height,
                    );
                    muted_color = self.calculate_color_fade(
                        muted_color,
                        i.saturating_sub(state.result_list_state.offset()),
                        available_height,
                    );
                }

                // construct line
                let line = Line::from(vec![
                    if self.settings.numbered {
                        // number index
                        Span::styled(
                            format!("{} ", prepend_icon),
                            Style::default()
                                .fg(theme.get_color("accent", Some(UISection::Results))),
                        )
                    } else {
                        Span::raw("")
                    },
                    Span::styled(padded_name.clone(), Style::default().fg(text_color)), // name
                    if self.settings.show_scores {
                        Span::styled(score_text.clone(), Style::default().fg(muted_color))
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
        StatefulWidget::render(list, inner_area, buf, &mut state.result_list_state);
        block.render(area, buf);
    }
}
