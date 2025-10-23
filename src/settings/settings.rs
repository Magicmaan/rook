use config::Config;
use dirs::config_dir;
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::model::module_state::UISection;
use crate::settings::serialise::{
    deserialize_alignment, deserialize_border_type, deserialize_color,
    deserialize_optional_border_type, deserialize_optional_color, serialize_alignment,
    serialize_border_type, serialize_color, serialize_optional_border_type,
    serialize_optional_color,
};
use crate::ui::util::IconMode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub struct KeybindSettings {
    pub quit: String,
    pub execute_search: String,
    pub left: String,
    pub right: String,
    pub up: String,
    pub down: String,
}
impl Default for KeybindSettings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            execute_search: "enter".into(),
            left: "left".into(),
            right: "right".into(),
            up: "up".into(),
            down: "down".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UISearchSettings {
    pub pre_query: String,     // text before the query input
    pub caret_text: String,    // caret character
    pub caret_blink_rate: u32, // in ms
    pub caret_visible: bool, // if disabled, remove blinking, caret, and care movement    // if true, search as you type
    #[serde(
        deserialize_with = "deserialize_alignment",
        serialize_with = "serialize_alignment"
    )]
    pub text_alignment: Alignment, // alignment of the text: left, center, right
    pub padding: u16,        // padding inside the search box
    pub rainbow_border: bool,
    pub rainbow_border_speed: f32, // speed of the rainbow border effect in scalar multiples 1.0, 1.5, 2.0 etc
}
impl Default for UISearchSettings {
    fn default() -> Self {
        Self {
            pre_query: ">>".into(),
            caret_text: "▋".into(),
            caret_blink_rate: 500,
            caret_visible: true,
            text_alignment: Alignment::Left,
            padding: 0,
            rainbow_border: false,
            rainbow_border_speed: 1.0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIResultsSettings {
    pub max_results: usize,         // maximum number of results to display
    pub show_scores: bool,          // whether to show scores next to results
    pub open_through_number: bool,  // whether to open results through number keybinds
    pub numbered: bool,             // whether to show numbers next to results
    pub number_mode: IconMode,      // icon mode for numbers
    pub loopback: bool,             // whether to loop back when navigating results
    pub fade_color_at_bottom: bool, // whether to fade text color towards the bottom
    pub padding: u16,               // padding inside the results box
    pub fade_in: bool,              // whether to fade in results on search
    pub fade_in_duration: u32,      // duration of fade in effect in ms
    pub fade_top_to_bottom: bool,   // pattern used for fade in effect
    pub rainbow_border: bool,
    pub rainbow_border_speed: f32, // speed of the rainbow border effect in scalar multiples 1.0, 1.5, 2.0 etc
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
            fade_color_at_bottom: true,   // fade text color towards the bottom. REQUIRES RGB COLORS
            padding: 1,
            fade_in: true,          // fade in results on search
            fade_in_duration: 1000, // duration of fade in effect in ms
            fade_top_to_bottom: true,
            rainbow_border: false,
            rainbow_border_speed: 1.0,
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
    pub padding: u16,             // padding around the entire UI
    pub title: String,            // title of the application
    #[serde(
        deserialize_with = "deserialize_alignment",
        serialize_with = "serialize_alignment"
    )]
    pub title_alignment: Alignment, // alignment of the title
}
impl Default for UILayoutSettings {
    fn default() -> Self {
        Self {
            sections: vec![UISection::Search, UISection::Results, UISection::Tooltip],
            gap: 1,
            padding: 1,
            title: "Rook".into(),
            title_alignment: Alignment::Center,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ThemeSettings {
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub background: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub highlight: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub muted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub muted_dark: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub accent: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub border: Color,

    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_muted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_accent: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub title: Color,

    #[serde(
        deserialize_with = "deserialize_border_type",
        serialize_with = "serialize_border_type"
    )]
    pub border_type: BorderType,

    search: SearchThemeSettings,

    results: ResultsThemeSettings,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(unused)]
pub struct SearchThemeSettings {
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub background: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub highlight: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub muted: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub muted_dark: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub accent: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub caret: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub border: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub pre_query_text: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub text: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub text_muted: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
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
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub background: Option<Color>,

    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub highlight: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub muted: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub muted_dark: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub accent: Option<Color>,

    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub border: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub text: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
    pub text_muted: Option<Color>,
    #[serde(
        deserialize_with = "deserialize_optional_color",
        serialize_with = "serialize_optional_color"
    )]
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
            highlight: Color::Yellow,
            muted: Color::DarkGray,
            muted_dark: Color::Black,
            accent: Color::Cyan,

            border: Color::Blue,

            text: Color::Rgb(200, 200, 200),
            text_muted: Color::Rgb(150, 150, 150),
            text_accent: Color::Cyan,
            title: Color::White,
            border_type: BorderType::Rounded,

            search: SearchThemeSettings {
                background: None,
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
            background: Some(self.search.background.unwrap_or(self.background)),
            highlight: Some(self.search.highlight.unwrap_or(self.highlight)),
            muted: Some(self.search.muted.unwrap_or(self.muted)),
            muted_dark: Some(self.search.muted_dark.unwrap_or(self.muted_dark)),
            accent: Some(self.search.accent.unwrap_or(self.accent)),
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
            background: Some(self.results.background.unwrap_or(self.background)),
            highlight: Some(self.results.highlight.unwrap_or(self.highlight)),
            muted: Some(self.results.muted.unwrap_or(self.muted)),
            muted_dark: Some(self.results.muted_dark.unwrap_or(self.muted_dark)),
            accent: Some(self.results.accent.unwrap_or(self.accent)),
            border: Some(self.results.border.unwrap_or(Color::Blue)),
            text: Some(self.results.text.unwrap_or(Color::Rgb(200, 200, 200))),
            text_muted: Some(self.results.text_muted.unwrap_or(Color::Rgb(150, 150, 150))),
            text_accent: Some(self.results.text_accent.unwrap_or(Color::Cyan)),
            border_type: Some(BorderType::Rounded),
        }
    }

    pub fn get_default_style(&self, section: Option<UISection>) -> Style {
        match section {
            Some(UISection::Search) => Style::default()
                .bg(self.search.background.unwrap_or(self.background))
                .fg(self.search.text.unwrap_or(self.text)),

            Some(UISection::Results) => Style::default()
                .bg(self.results.background.unwrap_or(self.background))
                .fg(self.results.text.unwrap_or(self.text)),

            _ => Style::default().bg(self.background).fg(self.text),
        }
    }

    pub fn get_default_border_style(&self, section: Option<UISection>) -> Style {
        match section {
            Some(UISection::Search) => {
                Style::default().fg(self.search.border.unwrap_or(self.border))
            }

            Some(UISection::Results) => {
                Style::default().fg(self.results.border.unwrap_or(self.border))
            }

            _ => Style::default().fg(self.border),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
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
            // Self::write_default_settings(config_file.clone());
            Self::read_settings(config_file);
        }

        Self::default()
    }

    fn read_settings(_config_file: PathBuf) -> Self {
        log::info!("Reading settings from {:?}", _config_file);

        let settings = Config::builder()
            .add_source(config::File::with_name(_config_file.to_str().unwrap()))
            // add default settings as fallback (errors if fields are missing)
            .add_source(config::File::from_str(
                &toml::to_string(&SearchSettings::default()).unwrap(),
                config::FileFormat::Toml,
            ))
            .add_source(config::File::from_str(
                &toml::to_string(&UISettings::default()).unwrap(),
                config::FileFormat::Toml,
            ))
            .add_source(config::File::from_str(
                &toml::to_string(&KeybindSettings::default()).unwrap(),
                config::FileFormat::Toml,
            ))
            .build()
            .expect("Could not build config from file");

        let structure: Settings = settings.try_deserialize().unwrap_or_else(|e| {
            log::error!(
                "Could not deserialize config file {:?} into Settings struct: {}",
                _config_file,
                e
            );
            Settings::default()
        });
        log::trace!("Deserialized settings: {:?}", structure);

        log::info!("Successfully built config from file {:?}", _config_file);
        structure
    }

    fn write_default_settings(&self, config_file: PathBuf) {
        std::fs::create_dir_all(
            config_file
                .parent()
                .expect("Could not get parent directory"),
        )
        .expect("Could not create config directory");

        // create default settings string from serializing self
        let string = toml::to_string_pretty(self).expect("Could not serialize default settings");
        std::fs::write(&config_file, string).expect("Could not write default settings to file");

        // std::fs::copy(&default_settings_path, &config_file)
        //     .expect("Could not copy default settings");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ftail::Ftail;
    use log::LevelFilter;

    #[test]
    fn test_write_default_settings() {
        let config_path = config_dir().expect("Could not find config directory");
        let config_file = config_path.join("rook").join("settings.toml");
        let settings = Settings::default();
        settings.write_default_settings(config_file.clone());

        log::info!("Settings struct: {:#?}", settings);
    }

    #[test]
    fn test_read_settings() {
        Ftail::new().console(LevelFilter::Trace).init().unwrap();
        let config_path = config_dir().expect("Could not find config directory");
        let config_file = config_path.join("rook").join("settings.toml");
        let settings = Settings::read_settings(config_file);

        // Option A: deserialize to a generic JSON-like value to inspect nested structure
        let value = settings;
        log::info!("Settings struct: {:#?}", value);
    }
}
