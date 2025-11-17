use std::{collections::HashMap, rc::Rc};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
};

use crate::{common::module_state::UISection, settings::settings::Settings};

pub struct RootLayout {
    pub search_box_area: Rect,
    pub results_box_area: Rect,
    pub wizard_box_area: Rect,
}

pub fn get_root_layout(area: Rect, settings: &Settings) -> RootLayout {
    let ui_settings = &settings.ui;
    let gap = ui_settings.layout.gap;

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
    let mut constraints = Vec::new();
    for (i, section) in ui_settings.layout.sections.iter().enumerate() {
        constraints.push(match section {
            UISection::Search => Constraint::Length(search_bar_height),
            UISection::Results => Constraint::Fill(0),
        });
        if i < ui_settings.layout.sections.len() - 1 {
            constraints.push(Constraint::Length(gap.saturating_sub(1)));
        }
    }

    // Split the area into chunks
    let h_layout = Layout::horizontal(vec![
        Constraint::Length(25),
        Constraint::Length(gap.saturating_sub(1)),
        Constraint::Fill(1),
    ])
    .split(area);
    let layout = Layout::vertical(constraints);
    let chunks = layout.split(h_layout[2]);

    // Map sections to their corresponding chunks (skip gap chunks)
    let mut section_areas = HashMap::new();
    for (i, section) in ui_settings.layout.sections.iter().enumerate() {
        let idx = i * 2;
        if let Some(chunk) = chunks.get(idx) {
            section_areas.insert(section, *chunk);
        }
    }

    RootLayout {
        search_box_area: *section_areas
            .get(&UISection::Search)
            .unwrap_or(&Rect::new(0, 0, 0, 0)),
        results_box_area: *section_areas
            .get(&UISection::Results)
            .unwrap_or(&Rect::new(0, 0, 0, 0)),
        wizard_box_area: h_layout[0],
    }
}
