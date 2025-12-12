use ratatui::{
    layout::Rect,
    style::Color,
    symbols::{self, border},
    widgets::Borders,
};
use serde::{Deserialize, Serialize};

use crate::{common::module_state::UISection, settings::settings::Settings};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconMode {
    Circle,
    Small,
    #[default]
    Normal,
    Subscript,
}
// small = ➀➁➂➃➄➅➆➇➈➉
// circle = ❶❷❸❹❺❻❼❽❾❿
// normal = 1 2 3 4 5 6 7 8 9 10

// im using font https://qwerasd205.github.io/PixelCode/index.html which have different looks
// small is just a small version of the number, like a subscript "₁" but not quite
// circle is a filled circle with the number inside
// normal is just the number itself
pub fn number_to_icon(number: usize, mode: IconMode) -> String {
    assert!(number > 0, "Number must be greater than 0");
    match (number, mode) {
        (1, IconMode::Circle) => "❶",
        (2, IconMode::Circle) => "❷",
        (3, IconMode::Circle) => "❸",
        (4, IconMode::Circle) => "❹",
        (5, IconMode::Circle) => "❺",
        (6, IconMode::Circle) => "❻",
        (7, IconMode::Circle) => "❼",
        (8, IconMode::Circle) => "❽",
        (9, IconMode::Circle) => "❾",
        (10, IconMode::Circle) => "❿",
        //
        (1, IconMode::Small) => "➀",
        (2, IconMode::Small) => "➁",
        (3, IconMode::Small) => "➂",
        (4, IconMode::Small) => "➃",
        (5, IconMode::Small) => "➄",
        (6, IconMode::Small) => "➅",
        (7, IconMode::Small) => "➆",
        (8, IconMode::Small) => "➇",
        (9, IconMode::Small) => "➈",
        (10, IconMode::Small) => "➉",
        //
        (1, IconMode::Normal) => "1",
        (2, IconMode::Normal) => "2",
        (3, IconMode::Normal) => "3",
        (4, IconMode::Normal) => "4",
        (5, IconMode::Normal) => "5",
        (6, IconMode::Normal) => "6",
        (7, IconMode::Normal) => "7",
        (8, IconMode::Normal) => "8",
        (9, IconMode::Normal) => "9",
        (10, IconMode::Normal) => "10",
        //
        (1, IconMode::Subscript) => "₁",
        (2, IconMode::Subscript) => "₂",
        (3, IconMode::Subscript) => "₃",
        (4, IconMode::Subscript) => "₄",
        (5, IconMode::Subscript) => "₅",
        (6, IconMode::Subscript) => "₆",
        (7, IconMode::Subscript) => "₇",
        (8, IconMode::Subscript) => "₈",
        (9, IconMode::Subscript) => "₉",
        (10, IconMode::Subscript) => "₁₀",
        _ => " ", // fallback for numbers > 10
    }
    .to_string()
}

pub fn collapsed_border(
    section: UISection,
    layout: &Vec<UISection>,
    default_border: border::Set,
) -> (Borders, border::Set) {
    let position = layout
        .iter()
        .position(|s| *s == section)
        .expect("Section not found in layout");

    let top_connected = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        ..default_border
    };
    let bottom_connected = symbols::border::Set {
        bottom_left: symbols::line::NORMAL.vertical_right,
        bottom_right: symbols::line::NORMAL.vertical_left,
        ..default_border
    };
    let both_connected = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.vertical_right,
        bottom_right: symbols::line::NORMAL.vertical_left,
        ..default_border
    };

    let len = layout.len();

    match position {
        0 => (
            Borders::LEFT | Borders::RIGHT | Borders::TOP,
            bottom_connected,
        ),
        middle if middle > 0 && middle < len - 1 => {
            (Borders::LEFT | Borders::RIGHT, both_connected)
        }
        _ => (Borders::ALL, top_connected),
    }
}

pub fn calculate_minimum_size(settings: &Settings) -> Rect {
    let mut min_width = 20;
    let mut min_height = 5;

    min_height += ((settings.ui.layout.gap.saturating_sub(1)) * 2) + 1;
    min_height += (settings.ui.layout.padding * 2) + 1;

    min_height += settings.ui.results.padding.saturating_sub(1) * 2;
    min_height += settings.ui.search.padding * 2;

    min_width += settings.ui.layout.padding * 2;
    min_width += settings.ui.search.padding * 2;
    min_width += settings.ui.results.padding.saturating_sub(1) * 2;
    // hardcoded minimum size for now
    // later can be calculated based on font size and ui settings
    Rect::new(0, 0, min_width, min_height)
}

pub fn loading_spinner(tick: u64) -> String {
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

pub fn multiply_color(color: Color, mult: f64) -> Color {
    if let Color::Rgb(r, g, b) = color {
        let r = (r as f64 * mult).round().min(255.0) as u8;
        let g = (g as f64 * mult).round().min(255.0) as u8;
        let b = (b as f64 * mult).round().min(255.0) as u8;

        Color::Rgb(r, g, b)
    } else {
        color
    }
}

pub fn calculate_color_fade(start_color: Color, position: usize, height: usize) -> Color {
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
            multiply_color(start_color, brightness as f64)
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
