use std::collections::HashMap;

use ratatui::{style::Color, widgets::BorderType};

use crate::model::ui::UISection;
use crate::ui::util::IconMode;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeybindSettings {
    pub quit: String,
    pub search: String,
    pub left: String,
    pub right: String,
    pub up: String,
    pub down: String,
}
impl Default for KeybindSettings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            search: "enter".into(),
            left: "left".into(),
            right: "right".into(),
            up: "up".into(),
            down: "down".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UISearchSettings {
    pub pre_query: String,     // text before the query input
    pub caret: String,         // caret character
    pub caret_blink_rate: u64, // in ms
    pub caret_visible: bool, // if disabled, remove blinking, caret, and care movement    // if true, search as you type
}
impl Default for UISearchSettings {
    fn default() -> Self {
        Self {
            pre_query: ">>".into(),
            caret: "▋".into(),
            caret_blink_rate: 500,
            caret_visible: true,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIResultsSettings {
    pub max_results: usize,        // maximum number of results to display
    pub show_scores: bool,         // whether to show scores next to results
    pub open_through_number: bool, // whether to open results through number keybinds
    pub numbered: bool,            // whether to show numbers next to results
    pub number_mode: IconMode,     // icon mode for numbers
    pub loopback: bool,            // whether to loop back when navigating results
    pub fade_color: bool,          // whether to fade text color towards the bottom
}
impl Default for UIResultsSettings {
    fn default() -> Self {
        Self {
            max_results: 20,
            show_scores: true,            // show fuzzy scores next to results
            numbered: true,               // show numbers next to results (upto 10)
            open_through_number: true,    // CTRL + number to open
            number_mode: IconMode::Small, // icon mode for numbers
            loopback: true,               // loop back when navigating results
            fade_color: true,             // fade text color towards the bottom. REQUIRES RGB COLORS
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UITooltipSettings {
    pub enabled: bool,     // whether tooltips are enabled
    pub max_width: usize,  // maximum width of tooltip
    pub max_height: usize, // maximum height of tooltip
    pub delay: u64,        // delay before showing tooltip in ms
}
impl Default for UITooltipSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_width: 50,
            max_height: 10,
            delay: 500,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UILayoutSettings {
    sections: Vec<UISection>, // order of layout sections
}
impl Default for UILayoutSettings {
    fn default() -> Self {
        Self {
            sections: vec![UISection::Search, UISection::Results, UISection::Tooltip],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeSettings {
    pub background: Color,
    pub foreground: Color,
    pub highlight: Color,
    pub muted: Color,
    pub muted_dark: Color,
    pub accent: Color,
    pub caret: Color,
    pub border: Color,

    pub text: Color,
    pub text_muted: Color,
    pub text_accent: Color,

    pub search_background: Option<Color>,
    pub search_foreground: Option<Color>,
    pub search_accent: Option<Color>,
    pub search_highlight: Option<Color>,
    pub search_muted: Option<Color>,
    pub search_muted_dark: Option<Color>,
    pub search_caret: Option<Color>,
    pub search_border: Option<Color>,

    pub results_background: Option<Color>,
    pub results_foreground: Option<Color>,
    pub results_muted: Option<Color>,
    pub results_muted_dark: Option<Color>,
    pub results_highlight: Option<Color>,
    pub results_accent: Option<Color>,
    pub results_caret: Option<Color>,
    pub results_border: Option<Color>,

    pub border_type: BorderType,
    pub search_border_type: Option<BorderType>,
    pub results_border_type: Option<BorderType>,
}
impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            background: Color::Reset,
            foreground: Color::White,
            highlight: Color::Yellow,
            muted: Color::DarkGray,
            muted_dark: Color::Black,
            accent: Color::Cyan,
            caret: Color::White,
            border: Color::Blue,

            text: Color::Rgb(200, 200, 200),
            text_muted: Color::Rgb(150, 150, 150),
            text_accent: Color::Cyan,

            search_background: None,
            search_foreground: None,
            search_highlight: None,
            search_muted: None,
            search_muted_dark: None,
            search_accent: None,
            search_caret: None,
            search_border: None,

            results_background: None,
            results_foreground: None,
            results_muted: None,
            results_muted_dark: None,
            results_highlight: None,
            results_accent: None,
            results_caret: None,
            results_border: None,

            border_type: BorderType::Rounded,
            search_border_type: None,
            results_border_type: None,
        }
    }
}
impl ThemeSettings {
    pub fn get_border_type(&self, section: &str) -> BorderType {
        match section {
            "search" => self.search_border_type.unwrap_or(self.border_type),
            "results" => self.results_border_type.unwrap_or(self.border_type),
            _ => self.border_type,
        }
    }

    pub fn get_colors(&self, section: Option<UISection>) -> HashMap<&str, Color> {
        let mut colors: HashMap<&str, Color> = HashMap::new();
        match section {
            Some(UISection::Search) => {
                colors.insert(
                    "background",
                    self.search_background
                        .clone()
                        .unwrap_or(self.background.clone()),
                );
                colors.insert(
                    "foreground",
                    self.search_foreground
                        .clone()
                        .unwrap_or(self.foreground.clone()),
                );
                colors.insert(
                    "highlight",
                    self.search_highlight
                        .clone()
                        .unwrap_or(self.highlight.clone()),
                );
                colors.insert(
                    "muted",
                    self.search_muted.clone().unwrap_or(self.muted.clone()),
                );
                colors.insert(
                    "muted_dark",
                    self.search_muted_dark
                        .clone()
                        .unwrap_or(self.muted_dark.clone()),
                );

                colors.insert(
                    "accent",
                    self.search_accent.clone().unwrap_or(self.accent.clone()),
                );
                colors.insert(
                    "caret",
                    self.search_caret.clone().unwrap_or(self.caret.clone()),
                );
                colors.insert(
                    "border",
                    self.search_border.clone().unwrap_or(self.border.clone()),
                );
            }
            Some(UISection::Results) => {
                colors.insert(
                    "background",
                    self.results_background
                        .clone()
                        .unwrap_or(self.background.clone()),
                );
                colors.insert(
                    "foreground",
                    self.results_foreground
                        .clone()
                        .unwrap_or(self.foreground.clone()),
                );
                colors.insert(
                    "highlight",
                    self.results_highlight
                        .clone()
                        .unwrap_or(self.highlight.clone()),
                );
                colors.insert(
                    "muted",
                    self.results_muted.clone().unwrap_or(self.muted.clone()),
                );
                colors.insert(
                    "muted_dark",
                    self.results_muted_dark
                        .clone()
                        .unwrap_or(self.muted_dark.clone()),
                );
                colors.insert(
                    "accent",
                    self.results_accent.clone().unwrap_or(self.accent.clone()),
                );
                colors.insert(
                    "caret",
                    self.results_caret.clone().unwrap_or(self.caret.clone()),
                );
                colors.insert(
                    "border",
                    self.results_border.clone().unwrap_or(self.border.clone()),
                );
            }
            _ => {
                colors.insert("background", self.background.clone());
                colors.insert("foreground", self.foreground.clone());
                colors.insert("highlight", self.highlight.clone());
                colors.insert("muted", self.muted.clone());
                colors.insert("muted_dark", self.muted_dark.clone());
                colors.insert("accent", self.accent.clone());
                colors.insert("caret", self.caret.clone());
                colors.insert("border", self.border.clone());
            }
        }
        colors.insert("text", self.text.clone());
        colors.insert("text_muted", self.text_muted.clone());
        colors.insert("text_accent", self.text_accent.clone());
        colors
    }

    pub fn get_color(&self, name: &str, section: Option<UISection>) -> Color {
        let colors = self.get_colors(section);
        colors
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("No color found for name: {}", name))
    }

    pub fn get_default_style(&self, section: Option<UISection>) -> ratatui::style::Style {
        let colors = self.get_colors(section.clone());

        ratatui::style::Style::default()
            .bg(*colors.get("background").unwrap_or(&Color::Black))
            .fg(*colors.get("foreground").unwrap_or(&Color::White))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UISettings {
    pub layout: UILayoutSettings,
    pub search: UISearchSettings,
    pub results: UIResultsSettings,
    pub tooltip: UITooltipSettings,
    pub theme: ThemeSettings,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchSettings {
    pub always_search: bool, // if true, search as you type
}
impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            always_search: true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Settings {
    // Add your settings fields here
    pub search: SearchSettings,
    pub ui: UISettings,
    pub keybinds: KeybindSettings,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}

//
// layout ]
//   "search"
//   "results"
//   "tooltip"
// [
//
// search {
//   pre_query: String, // text before the query input
//   caret: String,     // caret character
//   caret_blink_rate: u64, // in ms
//   caret_visible: bool, // if disabled, remove blinking, caret, and care movement
//   always_search: bool, // if true, search as you type
// }
//
// keybinds {
//    quit = "q"
//    search = "enter"
//    left = "left"
//    right = "right"
//    up = "up"
//    down = "down"
//
// }
//
// theme {
// 	  background = "Black"
//    foreground = "White"
//    highlight = "Yellow"
//    accent = "Cyan"
//    caret = "White"
//    border = "Blue"
//
//    search_background = "DarkGray"
//    search_foreground = "White"
//    search_accent = "Cyan"
//    search_caret = "White"
//    search_border = "Blue"
// }
