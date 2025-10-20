use crate::model::model::Model;
use crate::model::module::ModuleState;

use crate::settings;
use crate::settings::settings::{Settings, UIResultsSettings};
use crate::ui::util::number_to_icon;
use ratatui::symbols;
use ratatui::widgets::Borders;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding, StatefulWidget, Widget},
};

use crate::model::module::UISection;

#[derive(Clone)]
pub struct ResultsBox<'a> {
    settings: &'a Settings,
}

impl<'a> ResultsBox<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        Self { settings }
    }

    fn multiply_color(&self, color: Color, mult: f64) -> Color {
        if let Color::Rgb(r, g, b) = color {
            let r = (r as f64 * mult).round().min(255.0) as u8;
            let g = (g as f64 * mult).round().min(255.0) as u8;
            let b = (b as f64 * mult).round().min(255.0) as u8;

            Color::Rgb(r, g, b)
        } else {
            color
        }
    }

    fn calculate_color_fade(&self, start_color: Color, position: usize, height: usize) -> Color {
        if let Color::Rgb(r, g, b) = start_color {
            log::trace!(
                "Calculating color fade for position {} of {}",
                position,
                height
            );
            let diff = height.saturating_sub(position);
            if diff < 5 {
                let base_brightness = 1.0;
                let brightness = 1.0
                    - maths_rs::lerp(
                        base_brightness,
                        0.25,
                        (diff as f32 / height as f32).min(0.1).max(1.0),
                    );
                Color::Rgb(
                    (brightness * r as f32) as u8,
                    (brightness * g as f32) as u8,
                    (brightness * b as f32) as u8,
                )
            } else {
                start_color
            }
        } else {
            log::trace!(
                "Color fade not applied for non-RGB color at position {} of {}",
                position,
                height
            );
            // indexed / ANSI colours aren't supported for fine-grained fading, so just return the
            start_color
        }
    }

    fn get_loading_spinner(&self, tick: u64) -> String {
        let remainder = tick % 4;
        if remainder == 0 {
            "◜".to_string()
        } else if remainder == 1 {
            "◝".to_string()
        } else if remainder == 2 {
            "◞".to_string()
        } else {
            "◟".to_string()
        }
    }
}

impl<'a> StatefulWidget for ResultsBox<'a> {
    type State = ModuleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let results_settings: UIResultsSettings = self.settings.ui.results.clone();
        let theme = self.settings.ui.theme.clone();
        let results_theme = theme.get_results_colors();

        // if gap is 0, use the connected border set
        // look at: https://ratatui.rs/recipes/layout/collapse-borders/
        // i.e.
        // │       │
        // ├───────┤
        // │       │
        let gap = self.settings.ui.layout.gap;
        let borders = self
            .settings
            .ui
            .theme
            .get_border_type("results")
            .to_border_set();
        // replace top left and top right with vertical connectors
        let collapsed_borders = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..borders
        };
        let block = Block::bordered()
            .border_set(if gap > 0 { borders } else { collapsed_borders })
            .border_style(Style::default())
            // .border_type(theme.get_border_type("results"))
            .borders(Borders::ALL)
            // .title("Search")
            .padding(Padding::new(2, 2, 1, 1));

        // block.render(area, buf);
        let inner_area = block.inner(area);

        let available_height = inner_area.height as usize;

        // some random easing function to slow down the fade
        let time_since_search = ((state.tick.saturating_sub(state.search.last_search_tick) as f32)
            .powf(2.0)
            .powf(1.5)
            / 8.0) as u64;

        let mut results: Vec<(u16, usize)> = state.search.results.clone();

        if self.settings.ui.results.fade_previous_results
            && time_since_search < results.len() as u64
        {
            if results.len() > 0 {
                let _ = results.split_off((time_since_search as usize).max(results.len() - 1));
                let mut previous = state.search.previous_results.clone();
                if previous.len() > 0 {
                    previous = previous.split_off(time_since_search as usize);
                }

                results.extend_from_slice(&previous)
            }
        }

        let mut i = 1;
        let items = results
            .iter()
            .map(|(score, idx)| {
                // space out sections to fit the width
                let app = &state.data.applications[*idx];

                let score_text = format!("{}", score);

                // get number icon
                // mode configurable in settings
                let mut prepend_icon = number_to_icon(i, results_settings.number_mode);
                let executing_item = state.executing_item;
                // if executing, use loading spinner
                if executing_item.is_some() && i == executing_item.unwrap() + 1 {
                    prepend_icon = self.get_loading_spinner(state.tick);
                }

                // pad score to end i.e. "App Name       123"
                let line_width = inner_area.width as usize;
                let mut name_width = line_width.saturating_sub(score_text.len() - 1);
                if prepend_icon.trim().len() == 0 {
                    name_width = name_width.saturating_sub(4); // extra space if no icon
                } else {
                    name_width = name_width.saturating_sub(prepend_icon.len() + 1); // +1 for space
                }
                let padded_name = format!("{:<width$}", app.name, width = name_width);

                let mut text_color = results_theme.text.unwrap();
                let mut muted_color = results_theme.text_muted.unwrap();

                // calculate list color fade
                if results_settings.fade_color && available_height >= 10 {
                    log::trace!(
                        "Applying color fade for item {} at position {} of {}",
                        app.name,
                        i.saturating_sub(state.result_list_state.offset()),
                        available_height
                    );
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

                if self.settings.ui.results.fade_previous_results {
                    if i == time_since_search as usize {
                        text_color = Color::Rgb(0, 0, 255); // highlight newest result in blue
                    } else if i > time_since_search as usize {
                        text_color = self.multiply_color(
                            text_color,
                            1.0 - (time_since_search.max(10) as f64 / 10.0),
                        );
                        muted_color = self.multiply_color(muted_color, 0.25);
                    }
                }

                // construct line
                let line = Line::from(vec![
                    // number index
                    Span::styled(
                        format!("{} ", prepend_icon),
                        Style::default().fg(results_theme.accent.unwrap()),
                    ),
                    Span::styled(padded_name.clone(), Style::default().fg(text_color)), // name
                    if results_settings.show_scores {
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
            .highlight_style(Style::default().bg(Color::Blue));

        // list.render(inner_area, buf);
        StatefulWidget::render(list, inner_area, buf, &mut state.result_list_state);
        block.render(area, buf);
    }
}
