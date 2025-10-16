use std::collections::HashMap;

use config::Config;
use dirs::config_dir;
use ratatui::style::Style;
use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

use crate::model::ui::UISection;
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    sections: Vec<UISection>, // order of layout sections
}
impl Default for UILayoutSettings {
    fn default() -> Self {
        Self {
            sections: vec![UISection::Search, UISection::Results, UISection::Tooltip],
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
    pub caret: Color,
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
    pub caret: Option<Color>,
    #[serde(deserialize_with = "deserialize_optional_color")]
    pub border: Option<Color>,
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
            caret: Color::White,
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
                accent: None,
                caret: None,
                border: None,
                border_type: None,
            },

            results: ResultsThemeSettings {
                background: None,
                foreground: None,
                highlight: None,
                muted: None,
                muted_dark: None,
                accent: None,
                caret: None,
                border: None,
                border_type: None,
            },
        }
    }
}

fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match color {
        Color::Reset => "Reset",
        Color::Black => "Black",
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Yellow => "Yellow",
        Color::Blue => "Blue",
        Color::Magenta => "Magenta",
        Color::Cyan => "Cyan",
        Color::Gray => "Gray",
        Color::DarkGray => "DarkGray",
        Color::LightRed => "LightRed",
        Color::LightGreen => "LightGreen",
        Color::LightYellow => "LightYellow",
        Color::LightBlue => "LightBlue",
        Color::LightMagenta => "LightMagenta",
        Color::LightCyan => "LightCyan",
        Color::White => "White",
        Color::Rgb(r, g, b) => {
            return serializer.serialize_str(&format!("Rgb({},{},{})", r, g, b));
        }
        Color::Indexed(i) => return serializer.serialize_str(&format!("Indexed({})", i)),
    };
    serializer.serialize_str(s)
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    println!("Deserializing color: {}", s);
    match s.as_str() {
        "Reset" => Ok(Color::Reset),
        "Black" => Ok(Color::Black),
        "Red" => Ok(Color::Red),
        "Green" => Ok(Color::Green),
        "Yellow" => Ok(Color::Yellow),
        "Blue" => Ok(Color::Blue),
        "Magenta" => Ok(Color::Magenta),
        "Cyan" => Ok(Color::Cyan),
        "Gray" => Ok(Color::Gray),
        "DarkGray" => Ok(Color::DarkGray),
        "LightRed" => Ok(Color::LightRed),
        "LightGreen" => Ok(Color::LightGreen),
        "LightYellow" => Ok(Color::LightYellow),
        "LightBlue" => Ok(Color::LightBlue),
        "LightMagenta" => Ok(Color::LightMagenta),
        "LightCyan" => Ok(Color::LightCyan),
        "White" => Ok(Color::White),
        s if s.starts_with("Rgb(") && s.ends_with(")") => {
            let inner = &s[4..s.len() - 1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                let r = parts[0].parse().map_err(serde::de::Error::custom)?;
                let g = parts[1].parse().map_err(serde::de::Error::custom)?;
                let b = parts[2].parse().map_err(serde::de::Error::custom)?;
                Ok(Color::Rgb(r, g, b))
            } else {
                Err(serde::de::Error::custom("Invalid RGB format"))
            }
        }
        s if s.starts_with("Indexed(") && s.ends_with(")") => {
            let inner = &s[8..s.len() - 1];
            let index = inner.parse().map_err(serde::de::Error::custom)?;
            Ok(Color::Indexed(index))
        }
        _ => Ok(Color::Reset), // default fallback
    }
}

fn serialize_optional_color<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match color {
        Some(c) => serialize_color(c, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_optional_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    println!("Deserializing optional color: {:?}", opt);
    match opt {
        Some(s) if !s.is_empty() => Ok(Some(deserialize_color(
            serde::de::value::StringDeserializer::new(s),
        )?)),
        Some(_) => Ok(None),
        None => Ok(None),
    }
}

fn serialize_border_type<S>(border_type: &BorderType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match border_type {
        BorderType::Plain => "Plain",
        BorderType::Rounded => "Rounded",
        BorderType::Double => "Double",
        BorderType::Thick => "Thick",
        _ => "Rounded", // default fallback
    };
    serializer.serialize_str(s)
}

impl ThemeSettings {
    pub fn get_border_type(&self, section: &str) -> BorderType {
        match section {
            "search" => self.search.border_type.unwrap_or(self.border_type),
            "results" => self.results.border_type.unwrap_or(self.border_type),
            _ => self.border_type,
        }
    }

    pub fn get_color(&self, color_name: &str, section: Option<UISection>) -> Color {
        let colors = self.get_colors(section);
        *colors.get(color_name).unwrap_or(&Color::White)
    }

    pub fn get_colors(&self, section: Option<UISection>) -> HashMap<&str, Color> {
        let mut colors: HashMap<&str, Color> = HashMap::new();
        match section {
            Some(UISection::Search) => {
                colors.insert(
                    "background",
                    self.search
                        .background
                        .clone()
                        .unwrap_or(self.background.clone()),
                );
                colors.insert(
                    "foreground",
                    self.search
                        .foreground
                        .clone()
                        .unwrap_or(self.foreground.clone()),
                );
                colors.insert(
                    "highlight",
                    self.search
                        .highlight
                        .clone()
                        .unwrap_or(self.highlight.clone()),
                );
                colors.insert(
                    "muted",
                    self.search.muted.clone().unwrap_or(self.muted.clone()),
                );
                colors.insert(
                    "muted_dark",
                    self.search
                        .muted_dark
                        .clone()
                        .unwrap_or(self.muted_dark.clone()),
                );

                colors.insert(
                    "accent",
                    self.search.accent.clone().unwrap_or(self.accent.clone()),
                );
                colors.insert(
                    "caret",
                    self.search.caret.clone().unwrap_or(self.caret.clone()),
                );
                colors.insert(
                    "border",
                    self.search.border.clone().unwrap_or(self.border.clone()),
                );
            }
            Some(UISection::Results) => {
                colors.insert(
                    "background",
                    self.results
                        .background
                        .clone()
                        .unwrap_or(self.background.clone()),
                );
                colors.insert(
                    "foreground",
                    self.results
                        .foreground
                        .clone()
                        .unwrap_or(self.foreground.clone()),
                );
                colors.insert(
                    "highlight",
                    self.results
                        .highlight
                        .clone()
                        .unwrap_or(self.highlight.clone()),
                );
                colors.insert(
                    "muted",
                    self.results.muted.clone().unwrap_or(self.muted.clone()),
                );
                colors.insert(
                    "muted_dark",
                    self.results
                        .muted_dark
                        .clone()
                        .unwrap_or(self.muted_dark.clone()),
                );
                colors.insert(
                    "accent",
                    self.results.accent.clone().unwrap_or(self.accent.clone()),
                );
                colors.insert(
                    "caret",
                    self.results.caret.clone().unwrap_or(self.caret.clone()),
                );
                colors.insert(
                    "border",
                    self.results.border.clone().unwrap_or(self.border.clone()),
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

        colors
    }

    pub fn get_default_style(&self, section: Option<UISection>) -> Style {
        let colors = self.get_colors(section);
        Style::default()
            .bg(*colors.get("background").unwrap_or(&Color::Reset))
            .fg(*colors.get("foreground").unwrap_or(&Color::White))
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
        } else {
            println!("no config file found at {:?}", config_file);
            println!("generating default settings");
            Self::populate_settings(config_file);
        }

        Self::default()
    }

    fn read_settings(_config_file: PathBuf) -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name(_config_file.to_str().unwrap()))
            .build()
            .expect("Could not build config");

        let structure: Settings = settings
            .try_deserialize()
            .expect("Could not deserialize to Settings struct");
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

fn deserialize_border_type<'de, D>(deserializer: D) -> Result<BorderType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Plain" => Ok(BorderType::Plain),
        "Rounded" => Ok(BorderType::Rounded),
        "Double" => Ok(BorderType::Double),
        "Thick" => Ok(BorderType::Thick),
        _ => Ok(BorderType::Rounded), // default fallback
    }
}

fn serialize_optional_border_type<S>(
    border_type: &Option<BorderType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match border_type {
        Some(bt) => {
            let s = match bt {
                BorderType::Plain => "Plain",
                BorderType::Rounded => "Rounded",
                BorderType::Double => "Double",
                BorderType::Thick => "Thick",
                _ => "Rounded", // default fallback
            };
            serializer.serialize_some(s)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_optional_border_type<'de, D>(deserializer: D) -> Result<Option<BorderType>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => match s.as_str() {
            "Plain" => Ok(Some(BorderType::Plain)),
            "Rounded" => Ok(Some(BorderType::Rounded)),
            "Double" => Ok(Some(BorderType::Double)),
            "Thick" => Ok(Some(BorderType::Thick)),
            _ => Ok(Some(BorderType::Rounded)), // default fallback
        },
        None => Ok(None),
    }
}

// fn serialize_optional_border_type<S>(
//     border_type: &Option<BorderType>,
//     serializer: S,
// ) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     match border_type {
//         Some(bt) => {
//             let s = match bt {
//                 BorderType::Plain => "Plain",
//                 BorderType::Rounded => "Rounded",
//                 BorderType::Double => "Double",
//                 BorderType::Thick => "Thick",
//                 _ => "Rounded", // default fallback
//             };
//             serializer.serialize_some(s)
//         }
//         None => serializer.serialize_none(),
//     }
// }

// fn deserialize_optional_border_type<'de, D>(deserializer: D) -> Result<Option<BorderType>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let opt = Option::<String>::deserialize(deserializer)?;
//     match opt {
//         Some(s) => match s.as_str() {
//             "Plain" => Ok(Some(BorderType::Plain)),
//             "Rounded" => Ok(Some(BorderType::Rounded)),
//             "Double" => Ok(Some(BorderType::Double)),
//             "Thick" => Ok(Some(BorderType::Thick)),
//             _ => Ok(Some(BorderType::Rounded)), // default fallback
//         },
//         None => Ok(None),
//     }
// }

#[cfg(test)]
mod tests {
    use config::Config;

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
