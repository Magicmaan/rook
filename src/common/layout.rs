use std::{collections::HashMap, hash::Hash, rc::Rc};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect, Size},
};
use serde::{Deserialize, Serialize, ser::SerializeStruct};

use crate::{common::module_state::UISection, settings::settings::Settings};
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RootLayout {
    #[serde(skip)]
    pub left_right_split: u16,
    pub search_split: u16,

    pub search_box_area: Rect,
    pub results_box_area: Rect,
    pub wizard_box_area: Rect,
    need_update: bool,
    pub transitioning_left_right_split: bool,
    #[serde(skip)]
    target_left_right_split: u16,
}

impl Default for RootLayout {
    fn default() -> Self {
        Self {
            left_right_split: 25,
            search_split: 3,
            search_box_area: Rect::default(),
            results_box_area: Rect::default(),
            wizard_box_area: Rect::default(),
            need_update: true,
            transitioning_left_right_split: false,
            target_left_right_split: 25,
        }
    }
}

impl RootLayout {
    pub fn queue_update(&mut self) {
        self.need_update = true;
    }

    pub fn set_left_right_split(&mut self, split: u16) {
        // self.left_right_split = split;
        self.target_left_right_split = split;
        self.transitioning_left_right_split = true;
        self.queue_update();
    }

    pub fn calculate_split(&mut self, area: Rect, settings: &Settings) -> bool {
        let ui_settings = &settings.ui;
        let gap = ui_settings.layout.gap;

        if self.transitioning_left_right_split {
            let current_split = self.left_right_split;

            if current_split == self.target_left_right_split {
                self.transitioning_left_right_split = false;
            } else {
                // Simple linear transition for demonstration purposes
                let current_value = current_split;
                let target_value = self.target_left_right_split;

                let step = if current_value < target_value { 2 } else { -2 };
                let mut new_value = ((current_value as isize).saturating_add(step)) as u16;
                if (current_value as isize - target_value as isize).abs() < 2 {
                    new_value = target_value;
                }
                self.left_right_split = new_value;
            }

            self.queue_update();
        }

        // outer padding of app
        let padding = ui_settings.search.padding;
        let area = area.inner(Margin {
            vertical: ui_settings.layout.padding,
            horizontal: ui_settings.layout.padding * 2,
        });
        // Calculate search bar height
        let mut search_bar_height = 2 + padding.saturating_mul(2);
        if gap > 0 || ui_settings.layout.sections.first() != Some(&UISection::Search) {
            search_bar_height += 1;
        }

        // Build constraints for each section, inserting gaps between them
        let mut vertical_constraints = Vec::new();
        for (i, section) in ui_settings.layout.sections.iter().enumerate() {
            vertical_constraints.push(match section {
                UISection::Search => Constraint::Length(search_bar_height),
                UISection::Results => Constraint::Fill(1),
                _ => Constraint::Length(0),
            });
            if i < ui_settings.layout.sections.len() - 1 {
                vertical_constraints.push(Constraint::Length(gap.saturating_sub(1)));
            }
        }

        // Split the area into chunks
        let horizontal_layout = Layout::horizontal(vec![
            Constraint::Length(self.left_right_split),
            Constraint::Length(gap.saturating_sub(1)),
            Constraint::Fill(1),
        ])
        .split(area);

        let vertical_layout = Layout::vertical(vertical_constraints);
        let chunks = vertical_layout.split(horizontal_layout[2]);

        // Map sections to their corresponding chunks (skip gap chunks)
        let mut section_areas = HashMap::new();
        for (i, section) in ui_settings.layout.sections.iter().enumerate() {
            let idx = i * 2;
            if let Some(chunk) = chunks.get(idx) {
                section_areas.insert(section, *chunk);
            }
        }

        let search_box_area = *section_areas
            .get(&UISection::Search)
            .unwrap_or(&Rect::new(0, 0, 0, 0));
        let results_box_area = *section_areas
            .get(&UISection::Results)
            .unwrap_or(&Rect::new(0, 0, 0, 0));
        let wizard_box_area = horizontal_layout[0];

        if (self.search_box_area == search_box_area
            && self.results_box_area == results_box_area
            && self.wizard_box_area == wizard_box_area)
            || !self.need_update
        {
            self.need_update = false;
            return false;
        }

        self.search_box_area = *section_areas
            .get(&UISection::Search)
            .unwrap_or(&Rect::new(0, 0, 0, 0));
        self.results_box_area = *section_areas
            .get(&UISection::Results)
            .unwrap_or(&Rect::new(0, 0, 0, 0));
        self.wizard_box_area = horizontal_layout[0];
        true
    }
}

// pub fn get_root_layout(area: Rect, settings: &Settings) -> RootLayout {
//     let ui_settings = &settings.ui;
//     let gap = ui_settings.layout.gap;

//     // outer padding of app
//     let padding = ui_settings.search.padding;
//     let area = area.inner(Margin {
//         vertical: ui_settings.layout.padding,
//         horizontal: ui_settings.layout.padding * 2,
//     });
//     // Calculate search bar height
//     let mut search_bar_height = 2 + padding.saturating_mul(2);
//     if gap > 0 || ui_settings.layout.sections.first() != Some(&UISection::Search) {
//         search_bar_height += 1;
//     }

//     // Build constraints for each section, inserting gaps between them
//     let mut vertical_constraints = Vec::new();
//     for (i, section) in ui_settings.layout.sections.iter().enumerate() {
//         vertical_constraints.push(match section {
//             UISection::Search => Constraint::Length(search_bar_height),
//             UISection::Results => Constraint::Fill(1),
//             _ => Constraint::Length(0),
//         });
//         if i < ui_settings.layout.sections.len() - 1 {
//             vertical_constraints.push(Constraint::Length(gap.saturating_sub(1)));
//         }
//     }

//     // Split the area into chunks
//     let horizontal_layout = Layout::horizontal(vec![
//         Constraint::Length(25),
//         Constraint::Length(gap.saturating_sub(1)),
//         Constraint::Fill(1),
//     ])
//     .split(area);

//     let vertical_layout = Layout::vertical(vertical_constraints);
//     let chunks = vertical_layout.split(horizontal_layout[2]);

//     // Map sections to their corresponding chunks (skip gap chunks)
//     let mut section_areas = HashMap::new();
//     for (i, section) in ui_settings.layout.sections.iter().enumerate() {
//         let idx = i * 2;
//         if let Some(chunk) = chunks.get(idx) {
//             section_areas.insert(section, *chunk);
//         }
//     }

//     RootLayout {
//         search_box_area: *section_areas
//             .get(&UISection::Search)
//             .unwrap_or(&Rect::new(0, 0, 0, 0)),
//         results_box_area: *section_areas
//             .get(&UISection::Results)
//             .unwrap_or(&Rect::new(0, 0, 0, 0)),
//         wizard_box_area: horizontal_layout[0],
//     }
// }

pub fn calculate_minimum_size(settings: &Settings) -> Size {
    let ui_settings = &settings.ui;
    let ui_gap = ui_settings.layout.gap;
    Size {
        width: 80,
        height: 24,
    }
}
