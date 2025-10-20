use config::Config;
use dirs::config_dir;
use ratatui::style::Style;
use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::model::module::UISection;
use crate::settings::serialise::{
    deserialize_border_type, deserialize_color, deserialize_optional_border_type,
    deserialize_optional_color, serialize_optional_border_type,
};
use crate::ui::util::IconMode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UISearchSettings {
    pub pre_query: String,     // text before the query input
    pub caret_text: String,    // caret character
    pub caret_blink_rate: u64, // in ms
    pub caret_visible: bool, // if disabled, remove blinking, caret, and care movement    // if true, search as you type
}
impl Default for UISearchSettings {
    fn default() -> Self {
        Self {
            pre_query: ">>".into(),
            caret_text: "▋".into(),
            caret_blink_rate: 500,
            caret_visible: true,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UIResultsSettings {
    pub max_results: usize,        // maximum number of results to display
    pub show_scores: bool,         // whether to show scores next to results
    pub open_through_number: bool, // whether to open results through number keybinds
    pub numbered: bool,            // whether to show numbers next to results
    pub number_mode: IconMode,     // icon mode for numbers
    pub loopback: bool,            // whether to loop back when navigating results
    pub fade_color: bool,          // whether to fade text color towards the bottom
    pub fade_previous_results: bool,
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
            fade_previous_results: false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UILayoutSettings {
    pub sections: Vec<UISection>, // order of layout sections
    pub gap: u16,                 // gap between sections
}
impl Default for UILayoutSettings {
    fn default() -> Self {
        Self {
            sections: vec![UISection::Search, UISection::Results, UISection::Tooltip],
            gap: 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ThemeSettings {
    #[serde(deserialize_with = "deserialize_color")]
    pub background: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub highlight: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub muted: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub muted_dark: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub accent: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub border: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub text: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub text_muted: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub text_accent: Color,

    #[serde(deserialize_with = "deserialize_border_type")]
    pub border_type: BorderType,

    search: SearchThemeSettings,

    results: ResultsThemeSettings,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(unused)]
pub struct SearchThemeSettings {
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub background: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub foreground: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub highlight: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub muted: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub muted_dark: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub accent: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub caret: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub border: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub pre_query_text: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text_muted: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text_accent: Option<Color>,

    #[serde(
        serialize_with = "serialize_optional_border_type",
        deserialize_with = "deserialize_optional_border_type"
    )]
    pub border_type: Option<BorderType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(unused)]
pub struct ResultsThemeSettings {
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub background: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub foreground: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub highlight: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub muted: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub muted_dark: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub accent: Option<Color>,

    #[serde(deserialize_with = "deserialize_optional_color")]
    pub border: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text_muted: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub text_accent: Option<Color>,

    #[serde(
        serialize_with = "serialize_optional_border_type",
        deserialize_with = "deserialize_optional_border_type"
    )]
    pub border_type: Option<BorderType>,
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

            border: Color::Blue,

            text: Color::Rgb(200, 200, 200),
            text_muted: Color::Rgb(150, 150, 150),
            text_accent: Color::Cyan,
            border_type: BorderType::Rounded,

            search: SearchThemeSettings {
                background: None,
                foreground: None,
                highlight: None,
                muted: None,
                muted_dark: None,
                pre_query_text: None,
                text: None,
                text_muted: None,
                text_accent: None,
                accent: None,
                caret: Some(Color::Yellow),
                border: None,
                border_type: None,
            },

            results: ResultsThemeSettings {
                background: None,
                foreground: None,
                highlight: None,
                muted: None,
                muted_dark: None,
                text: None,
                text_muted: None,
                text_accent: None,
                accent: None,

                border: None,
                border_type: None,
            },
        }
    }
}

impl ThemeSettings {
    pub fn get_border_type(&self, section: &str) -> BorderType {
        match section {
            "search" => self.search.border_type.unwrap_or(self.border_type),
            "results" => self.results.border_type.unwrap_or(self.border_type),
            _ => self.border_type,
        }
    }

    pub fn get_search_colors(&self) -> SearchThemeSettings {
        SearchThemeSettings {
            background: Some(self.search.background.unwrap_or(self.background.clone())),
            foreground: Some(self.search.foreground.unwrap_or(self.foreground.clone())),
            highlight: Some(self.search.highlight.unwrap_or(self.highlight.clone())),
            muted: Some(self.search.muted.unwrap_or(self.muted.clone())),
            muted_dark: Some(self.search.muted_dark.unwrap_or(self.muted_dark.clone())),
            accent: Some(self.search.accent.unwrap_or(self.accent.clone())),
            caret: Some(self.search.caret.unwrap_or(Color::Yellow)),
            border: Some(self.search.border.unwrap_or(Color::Blue)),
            pre_query_text: Some(
                self.search
                    .pre_query_text
                    .unwrap_or(Color::Rgb(200, 200, 200)),
            ),
            text: Some(self.search.text.unwrap_or(Color::Rgb(200, 200, 200))),
            text_muted: Some(self.search.text_muted.unwrap_or(Color::Rgb(150, 150, 150))),
            text_accent: Some(self.search.text_accent.unwrap_or(Color::Cyan)),
            border_type: Some(BorderType::Rounded),
        }
    }
    pub fn get_results_colors(&self) -> ResultsThemeSettings {
        ResultsThemeSettings {
            background: Some(self.results.background.unwrap_or(self.background.clone())),
            foreground: Some(self.results.foreground.unwrap_or(self.foreground.clone())),
            highlight: Some(self.results.highlight.unwrap_or(self.highlight.clone())),
            muted: Some(self.results.muted.unwrap_or(self.muted.clone())),
            muted_dark: Some(self.results.muted_dark.unwrap_or(self.muted_dark.clone())),
            accent: Some(self.results.accent.unwrap_or(self.accent.clone())),
            border: Some(self.results.border.unwrap_or(Color::Blue)),
            text: Some(self.results.text.unwrap_or(Color::Rgb(200, 200, 200))),
            text_muted: Some(self.results.text_muted.unwrap_or(Color::Rgb(150, 150, 150))),
            text_accent: Some(self.results.text_accent.unwrap_or(Color::Cyan)),
            border_type: Some(BorderType::Rounded),
        }
    }

    // pub fn get_color(&self, color_name: &str, section: Option<UISection>) -> Color {
    //     let colors = self.get_colors(section);
    //     *colors.get(color_name).unwrap_or(&Color::White)
    // }

    // pub fn get_colors(&self, section: Option<UISection>) -> HashMap<&str, Color> {
    //     let mut colors: HashMap<&str, Color> = HashMap::new();
    //     match section {
    //         Some(UISection::Search) => {
    //             colors.insert(
    //                 "background",
    //                 self.search
    //                     .background
    //                     .clone()
    //                     .unwrap_or(self.background.clone()),
    //             );
    //             colors.insert(
    //                 "foreground",
    //                 self.search
    //                     .foreground
    //                     .clone()
    //                     .unwrap_or(self.foreground.clone()),
    //             );
    //             colors.insert(
    //                 "highlight",
    //                 self.search
    //                     .highlight
    //                     .clone()
    //                     .unwrap_or(self.highlight.clone()),
    //             );
    //             colors.insert(
    //                 "muted",
    //                 self.search.muted.clone().unwrap_or(self.muted.clone()),
    //             );
    //             colors.insert(
    //                 "muted_dark",
    //                 self.search
    //                     .muted_dark
    //                     .clone()
    //                     .unwrap_or(self.muted_dark.clone()),
    //             );

    //             colors.insert(
    //                 "accent",
    //                 self.search.accent.clone().unwrap_or(self.accent.clone()),
    //             );

    //             colors.insert(
    //                 "border",
    //                 self.search.border.clone().unwrap_or(self.border.clone()),
    //             );
    //             colors.insert(
    //                 "pre_query_text",
    //                 self.search
    //                     .pre_query_text
    //                     .clone()
    //                     .unwrap_or(self.text.clone()),
    //             );
    //             colors.insert(
    //                 "text",
    //                 self.search.text.clone().unwrap_or(self.text.clone()),
    //             );
    //             colors.insert(
    //                 "text_muted",
    //                 self.search
    //                     .text_muted
    //                     .clone()
    //                     .unwrap_or(self.text_muted.clone()),
    //             );
    //             colors.insert(
    //                 "text_accent",
    //                 self.search
    //                     .text_accent
    //                     .clone()
    //                     .unwrap_or(self.text_accent.clone()),
    //             );
    //         }
    //         Some(UISection::Results) => {
    //             colors.insert(
    //                 "background",
    //                 self.results
    //                     .background
    //                     .clone()
    //                     .unwrap_or(self.background.clone()),
    //             );
    //             colors.insert(
    //                 "foreground",
    //                 self.results
    //                     .foreground
    //                     .clone()
    //                     .unwrap_or(self.foreground.clone()),
    //             );
    //             colors.insert(
    //                 "highlight",
    //                 self.results
    //                     .highlight
    //                     .clone()
    //                     .unwrap_or(self.highlight.clone()),
    //             );
    //             colors.insert(
    //                 "muted",
    //                 self.results.muted.clone().unwrap_or(self.muted.clone()),
    //             );
    //             colors.insert(
    //                 "muted_dark",
    //                 self.results
    //                     .muted_dark
    //                     .clone()
    //                     .unwrap_or(self.muted_dark.clone()),
    //             );
    //             colors.insert(
    //                 "accent",
    //                 self.results.accent.clone().unwrap_or(self.accent.clone()),
    //             );

    //             colors.insert(
    //                 "border",
    //                 self.results.border.clone().unwrap_or(self.border.clone()),
    //             );
    //             colors.insert(
    //                 "text",
    //                 self.results.text.clone().unwrap_or(self.text.clone()),
    //             );
    //             colors.insert(
    //                 "text_muted",
    //                 self.results
    //                     .text_muted
    //                     .clone()
    //                     .unwrap_or(self.text_muted.clone()),
    //             );
    //             colors.insert(
    //                 "text_accent",
    //                 self.results
    //                     .text_accent
    //                     .clone()
    //                     .unwrap_or(self.text_accent.clone()),
    //             );
    //         }
    //         _ => {
    //             colors.insert("background", self.background.clone());
    //             colors.insert("foreground", self.foreground.clone());
    //             colors.insert("highlight", self.highlight.clone());
    //             colors.insert("muted", self.muted.clone());
    //             colors.insert("muted_dark", self.muted_dark.clone());
    //             colors.insert("accent", self.accent.clone());
    //             colors.insert("border", self.border.clone());
    //             colors.insert("text", self.text.clone());
    //             colors.insert("text_muted", self.text_muted.clone());
    //             colors.insert("text_accent", self.text_accent.clone());
    //         }
    //     }

    //     colors
    // }

    pub fn get_default_style(&self) -> Style {
        Style::default()
            .bg(self.background.clone())
            .fg(self.foreground.clone())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct UISettings {
    pub layout: UILayoutSettings,
    pub search: UISearchSettings,
    pub results: UIResultsSettings,
    pub tooltip: UITooltipSettings,
    pub theme: ThemeSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Settings {
    // Add your settings fields here
    pub search: SearchSettings,
    pub ui: UISettings,
    pub keybinds: KeybindSettings,
}

impl Settings {
    pub fn new() -> Self {
        let path = config_dir().expect("Could not find config directory");
        let config_file = path.join("rook").join("settings.toml");

        if config_file.exists() {
            println!("loading settings from {:?}", config_file);
            return Self::read_settings(config_file);
        } else {
            println!("no config file found at {:?}", config_file);
            println!("generating default settings");
            Self::populate_settings(config_file.clone());
            Self::read_settings(config_file);
        }

        Self::default()
    }

    fn read_settings(_config_file: PathBuf) -> Self {
        log::info!("Reading settings from {:?}", _config_file);
        let settings = Config::builder()
            .add_source(config::File::with_name(_config_file.to_str().unwrap()))
            .build()
            .unwrap_or_else(|e| {
                log::error!("Could not build config from file {:?}: {}", _config_file, e);
                panic!("Could not build config from file {:?}: {}", _config_file, e)
            });

        let structure: Settings = settings.try_deserialize().unwrap_or_else(|e| {
            log::error!(
                "Could not deserialize config file {:?} into Settings struct: {}",
                _config_file,
                e
            );
            panic!(
                "Could not deserialize config file {:?} into Settings struct: {}",
                _config_file, e
            )
        });
        log::trace!("Deserialized settings: {:?}", structure);

        log::info!("Successfully built config from file {:?}", _config_file);
        structure
    }

    fn populate_settings(config_file: PathBuf) {
        // write default settings to file
        let default_settings_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("settings")
            .join("default_settings.toml");
        if !default_settings_path.exists() {
            panic!(
                "Default settings file not found at {:?}",
                default_settings_path
            );
        }
        std::fs::create_dir_all(
            config_file
                .parent()
                .expect("Could not get parent directory"),
        )
        .expect("Could not create config directory");

        std::fs::copy(&default_settings_path, &config_file)
            .expect("Could not copy default settings");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_write_default_settings() {
        let config_path = config_dir().expect("Could not find config directory");
        let config_file = config_path.join("rook").join("settings.toml");
        std::fs::remove_file(&config_file).ok(); // remove if exists
        Settings::populate_settings(config_file.clone());
        assert!(config_file.exists());
    }

    #[test]
    fn test_read_settings() {
        let config_path = config_dir().expect("Could not find config directory");
        let config_file = config_path.join("rook").join("settings.toml");
        let settings = Settings::read_settings(config_file);

        // Option A: deserialize to a generic JSON-like value to inspect nested structure
        let value = settings;
        println!("Settings struct: {:#?}", value);
    }
}
