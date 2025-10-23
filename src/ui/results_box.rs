use crate::effects;
use crate::model::module_state::{Result, UISection};

use crate::settings::settings::{Settings, UIResultsSettings};
use crate::ui::util::{IconMode, collapsed_border, number_to_icon};
use ratatui::layout::Margin;
use ratatui::symbols;
use ratatui::widgets::{Borders, ListState};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Padding, StatefulWidget, Widget},
};
use serde_json::Number;
use tachyonfx::{Duration, EffectManager, EffectTimer, Interpolation, fx, pattern};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ResultBoxState {
    pub results: Vec<Result>,
    pub previous_results: Vec<Result>,

    pub executing_item: Option<usize>,
    pub list_state: ListState,
    pub last_search_tick: u64,
    pub tick: u64,
    pub delta_time: i32,
}

#[derive(Clone)]
pub struct ResultsBox {
    settings: Settings,
}

impl ResultsBox {
    pub fn new(settings: &Settings) -> Self {
        Self {
            settings: settings.clone(),
        }
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
        if let Color::Rgb(_, _, _) = start_color {
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
                        (diff as f32 / height as f32).clamp(0.1, 1.0),
                    );
                self.multiply_color(start_color, brightness as f64)
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

    pub fn construct_list(
        &self,
        results: &Vec<Result>,
        number_mode: IconMode,
        executing_item: Option<usize>,
        state: &ResultBoxState,
        area: Rect,
        tick: u64,
    ) -> Vec<ListItem<'static>> {
        let theme = self.settings.ui.theme.get_results_colors();
        let available_height = area.height as usize;
        let mut i = 1;
        let items: Vec<ListItem<'static>> = results
            .iter()
            // .map(|(score, idx)| {
            .map(|r| {
                let result = &r.result;
                let score = &r.score;

                // space out sections to fit the width
                // let app = &state.data.applications[*idx];

                let score_text = score.to_string();

                // get number icon
                // mode configurable in settings
                let mut prepend_icon = number_to_icon(i, number_mode);
                // let executing_item = state.executing_item;
                // if executing, use loading spinner
                if executing_item.is_some() && i == executing_item.unwrap() + 1 {
                    prepend_icon = self.get_loading_spinner(tick);
                }

                // pad score to end i.e. "App Name       123"
                let line_width = area.width as usize;
                let mut name_width = line_width.saturating_sub(score_text.len() - 1);
                if prepend_icon.trim().is_empty() {
                    name_width = name_width.saturating_sub(4); // extra space if no icon
                } else {
                    name_width = name_width.saturating_sub(prepend_icon.len() + 1); // +1 for space
                }
                let padded_name = format!("{:<width$}", result, width = name_width);

                let mut text_color = theme.text.unwrap();
                let mut muted_color = theme.text_muted.unwrap();

                // calculate list color fade
                if self.settings.ui.results.fade_color_at_bottom && available_height >= 10 {
                    text_color = self.calculate_color_fade(
                        theme.text.unwrap(),
                        i.saturating_sub(state.list_state.offset()),
                        available_height,
                    );
                    muted_color = self.calculate_color_fade(
                        theme.text_muted.unwrap(),
                        i.saturating_sub(state.list_state.offset()),
                        available_height,
                    );
                }

                // construct line
                let line = Line::from(vec![
                    // number index
                    Span::styled(
                        format!("{} ", prepend_icon),
                        Style::default().fg(theme.accent.unwrap()),
                    ),
                    Span::styled(padded_name.clone(), Style::default().fg(text_color)), // name
                    if self.settings.ui.results.show_scores {
                        Span::styled(score_text.clone(), Style::default().fg(muted_color))
                    } else {
                        Span::raw("")
                    },
                ]);
                i += 1;
                ListItem::new(line)
            })
            .collect::<Vec<ListItem>>();

        items
    }
}

impl StatefulWidget for ResultsBox {
    type State<'b> = ResultBoxState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State<'_>) {
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
        let padding = results_settings.padding;
        let default_borders = theme.get_border_type(UISection::Results).to_border_set();

        let collapsed_borders = collapsed_border(UISection::Results, default_borders);
        // replace top left and top right with vertical connectors
        let collapsed_borders = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..default_borders
        };
        let block = Block::bordered()
            .border_set(if gap > 0 {
                default_borders
            } else {
                collapsed_borders
            })
            .border_style(
                self.settings
                    .ui
                    .theme
                    .get_default_border_style(Some(UISection::Results)),
            )
            // .border_type(theme.get_border_type("results"))
            .borders(Borders::ALL)
            // .title("Search")
            .padding(Padding::new(
                padding.saturating_mul(2).max(1),
                padding.saturating_mul(2).max(1),
                padding,
                padding,
            ))
            .style(theme.get_default_style(Some(UISection::Results)));

        // block.render(area, buf);
        let inner_area = block.inner(area);

        block.render(area, buf);

        // rainbow border effect
        if self.settings.ui.results.rainbow_border {
            let t = state.tick as u32;
            let speed: f32 = self.settings.ui.results.rainbow_border_speed;
            effects::rainbow(results_theme.border.unwrap(), 2000, speed, area, buf, t);
        }

        let results: &Vec<Result> = &state.results;

        let items = self.construct_list(
            results,
            self.settings.ui.results.number_mode,
            state.executing_item,
            state,
            inner_area,
            state.tick,
        );

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_symbol("")
            .highlight_style(Style::default().bg(results_theme.highlight.unwrap()));

        // render list with state
        StatefulWidget::render(list, inner_area, buf, &mut state.list_state);

        // fade in effect
        if self.settings.ui.results.fade_in {
            let mut direction: Option<pattern::AnyPattern> = None;
            if results_settings.fade_top_to_bottom {
                direction = Some(pattern::AnyPattern::Sweep(
                    pattern::SweepPattern::down_to_up(area.height as u16),
                ));
            }
            let tick =
                state.tick.saturating_sub(state.last_search_tick) as u32 * state.delta_time as u32;

            effects::fade_in(
                Color::Black,
                self.settings.ui.results.fade_in_duration,
                direction,
                inner_area,
                buf,
                tick,
            );
        }
    }
}
