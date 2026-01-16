use config::Config;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use dirs::config_dir;
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::{style::Color, widgets::BorderType};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::action::{Action, Search};
use crate::common::module_state::UISection;
use crate::components::util::IconMode;
use crate::settings::serialise::{
    deserialize_alignment, deserialize_border_type, deserialize_color,
    deserialize_optional_border_type, deserialize_optional_color, serialize_alignment,
    serialize_border_type, serialize_color, serialize_optional_border_type,
    serialize_optional_color,
};

pub fn get_settings_path() -> PathBuf {
    let path = config_dir().expect("Could not find config directory");
    path.join("rook")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializableKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}
impl Default for SerializableKeyEvent {
    fn default() -> Self {
        Self {
            code: KeyCode::Null,
            modifiers: KeyModifiers::NONE,
        }
    }
}
impl Serialize for SerializableKeyEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut parts = Vec::new();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl");
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt");
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift");
        }
        if self.modifiers.contains(KeyModifiers::SUPER) {
            parts.push("Super");
        }

        if (self.code == KeyCode::Null) {
            return serializer.serialize_none();
        }

        let key_str = match self.code {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            KeyCode::Null => "".to_string(),
            _ => format!("{:?}", self.code),
        };

        if !parts.is_empty() {
            parts.push(&key_str);
            serializer.serialize_str(&parts.join(" + "))
        } else {
            serializer.serialize_str(&key_str)
        }
    }
}

impl<'de> Deserialize<'de> for SerializableKeyEvent {
    // Deserialize a KeyEvent from a string like "Ctrl + A" or "Enter"
    // can be in capitals or small letters
    // splits by "+", trims spaces
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split("+").map(|part| part.trim()).collect();

        let mut modifiers = KeyModifiers::empty();
        let key_part = parts.last().ok_or_else(|| {
            serde::de::Error::custom("Invalid key event format: missing key part")
        })?;

        for &part in &parts[..parts.len() - 1] {
            match part.to_lowercase().as_str() {
                "ctrl" => modifiers |= KeyModifiers::CONTROL,
                "alt" => modifiers |= KeyModifiers::ALT,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                "super" => modifiers |= KeyModifiers::SUPER,
                _ => {}
            }
        }

        let code = match key_part.to_lowercase().as_str() {
            "enter" => KeyCode::Enter,
            "tab" => KeyCode::Tab,
            "backspace" => KeyCode::Backspace,
            "esc" => KeyCode::Esc,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            k if k.starts_with('f') => {
                let n: u8 = k[1..]
                    .parse()
                    .map_err(|_| serde::de::Error::custom("Invalid function key number"))?;
                KeyCode::F(n)
            }
            k if k.len() == 1 => KeyCode::Char(k.chars().next().unwrap()),
            _ => return Err(serde::de::Error::custom("Unknown key code")),
        };

        Ok(SerializableKeyEvent { code, modifiers })
    }
}

impl From<KeyEvent> for SerializableKeyEvent {
    fn from(event: KeyEvent) -> Self {
        SerializableKeyEvent {
            code: event.code,
            modifiers: event.modifiers,
        }
    }
}

impl From<&SerializableKeyEvent> for KeyEvent {
    fn from(event: &SerializableKeyEvent) -> Self {
        KeyEvent::new(event.code, event.modifiers)
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
    pub max_results: usize,           // maximum number of results to display
    pub show_scores: bool,            // whether to show scores next to results
    pub open_through_number: bool,    // whether to open results through number keybinds
    pub numbered: bool,               // whether to show numbers next to results
    pub number_mode: IconMode,        // icon mode for numbers
    pub loopback: bool,               // whether to loop back when navigating results
    pub fade_color_at_bottom: bool,   // whether to fade text color towards the bottom
    pub padding: u16,                 // padding inside the results box
    pub fade_in: bool,                // whether to fade in results on search
    pub fade_in_duration: u32,        // duration of fade in effect in ms
    pub fade_top_to_bottom: bool,     // pattern used for fade in effect
    pub rainbow_border: bool,         // whether to use rainbow border effect
    pub rainbow_border_speed: f32, // speed of the rainbow border effect in scalar multiples 1.0, 1.5, 2.0 etc
    pub show_number_of_results: bool, // whether to show number of results at the top
    pub number_of_results_position: VerticalAlignment, // position of number of results text
    #[serde(
        deserialize_with = "deserialize_alignment",
        serialize_with = "serialize_alignment"
    )]
    pub number_of_results_alignment: Alignment, // alignment of number of results text
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
            show_number_of_results: true,
            number_of_results_position: VerticalAlignment::Bottom,
            number_of_results_alignment: Alignment::Right,
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
            sections: vec![UISection::Search, UISection::Results],
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
    #[serde(default)]
    pub background: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub highlight: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub muted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub muted_dark: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub accent: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub border: Color,

    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub text: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub text_muted: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub text_accent: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    #[serde(default)]
    pub title: Color,

    #[serde(
        deserialize_with = "deserialize_border_type",
        serialize_with = "serialize_border_type"
    )]
    #[serde(default)]
    pub border_type: BorderType,
    #[serde(default)]
    search: Option<SearchThemeSettings>,
    #[serde(default)]
    results: Option<ResultsThemeSettings>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[allow(unused)]
#[serde(default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[allow(unused)]
#[serde(default)]
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
            highlight: Color::Blue,
            muted: Color::DarkGray,
            muted_dark: Color::Black,
            accent: Color::Cyan,

            border: Color::Blue,

            text: Color::Rgb(200, 200, 200),
            text_muted: Color::Rgb(150, 150, 150),
            text_accent: Color::Cyan,
            title: Color::White,
            border_type: BorderType::Rounded,

            search: Some(SearchThemeSettings::default()),

            results: Some(ResultsThemeSettings::default()),
        }
    }
}

impl ThemeSettings {
    pub fn get_border_type(&self, section: UISection) -> BorderType {
        let search = self.search.clone().unwrap_or_default();
        let results = self.results.clone().unwrap_or_default();
        match section {
            UISection::Search => search.border_type.unwrap_or(self.border_type),
            UISection::Results => results.border_type.unwrap_or(self.border_type),
            _ => self.border_type,
        }
    }

    pub fn get_search_colors(&self) -> SearchThemeSettings {
        let search = self.search.clone().unwrap_or_default();
        SearchThemeSettings {
            background: Some(search.background.unwrap_or(self.background)),
            highlight: Some(search.highlight.unwrap_or(self.highlight)),
            muted: Some(search.muted.unwrap_or(self.muted)),
            muted_dark: Some(search.muted_dark.unwrap_or(self.muted_dark)),
            accent: Some(search.accent.unwrap_or(self.accent)),
            caret: Some(search.caret.unwrap_or(Color::Yellow)),
            border: Some(search.border.unwrap_or(Color::Blue)),
            pre_query_text: Some(search.pre_query_text.unwrap_or(Color::Rgb(200, 200, 200))),
            text: Some(search.text.unwrap_or(Color::Rgb(200, 200, 200))),
            text_muted: Some(search.text_muted.unwrap_or(Color::Rgb(150, 150, 150))),
            text_accent: Some(search.text_accent.unwrap_or(Color::Cyan)),
            border_type: Some(BorderType::Rounded),
        }
    }
    pub fn get_results_colors(&self) -> ResultsThemeSettings {
        let results = self.results.clone().unwrap_or_default();
        ResultsThemeSettings {
            background: Some(results.background.unwrap_or(self.background)),
            highlight: Some(results.highlight.unwrap_or(self.highlight)),
            muted: Some(results.muted.unwrap_or(self.muted)),
            muted_dark: Some(results.muted_dark.unwrap_or(self.muted_dark)),
            accent: Some(results.accent.unwrap_or(self.accent)),
            border: Some(results.border.unwrap_or(Color::Blue)),
            text: Some(results.text.unwrap_or(Color::Rgb(200, 200, 200))),
            text_muted: Some(results.text_muted.unwrap_or(Color::Rgb(150, 150, 150))),
            text_accent: Some(results.text_accent.unwrap_or(Color::Cyan)),
            border_type: Some(BorderType::Rounded),
        }
    }

    pub fn get_default_style(&self, section: Option<UISection>) -> Style {
        let search_style = self.get_search_colors();
        let results_style = self.get_results_colors();
        match section {
            Some(UISection::Search) => Style::default()
                .bg(search_style.background.unwrap_or(self.background))
                .fg(search_style.text.unwrap_or(self.text)),
            Some(UISection::Results) => Style::default()
                .bg(results_style.background.unwrap_or(self.background))
                .fg(results_style.text.unwrap_or(self.text)),

            _ => Style::default().bg(self.background).fg(self.text),
        }
    }

    pub fn get_default_border_style(&self, section: Option<UISection>) -> Style {
        let search_style = self.get_search_colors();
        let results_style = self.get_results_colors();
        match section {
            Some(UISection::Search) => {
                Style::default().fg(search_style.border.unwrap_or(self.border))
            }

            Some(UISection::Results) => {
                Style::default().fg(results_style.border.unwrap_or(self.border))
            }

            _ => Style::default().fg(self.border),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct UISettings {
    #[serde(default)]
    pub layout: UILayoutSettings,
    #[serde(default)]
    pub search: UISearchSettings,
    #[serde(default)]
    pub results: UIResultsSettings,
    #[serde(default)]
    pub tooltip: UITooltipSettings,
    #[serde(default)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyBindings {
    #[serde(default)]
    quit: SerializableKeyEvent,
    #[serde(default)]
    navigate_down: SerializableKeyEvent,
    #[serde(default)]
    navigate_up: SerializableKeyEvent,
    #[serde(default)]
    navigate_left: SerializableKeyEvent,
    #[serde(default)]
    navigate_right: SerializableKeyEvent,
    #[serde(default)]
    navigate_home: SerializableKeyEvent,
    #[serde(default)]
    navigate_end: SerializableKeyEvent,
    #[serde(default)]
    focus_next: SerializableKeyEvent,
    #[serde(default)]
    focus_previous: SerializableKeyEvent,
}
impl KeyBindings {
    pub fn get_event_mapping(&self) -> HashMap<SerializableKeyEvent, Action> {
        let str = toml::to_string(&self.clone()).unwrap_or_default();

        let mapping: HashMap<Action, SerializableKeyEvent> = toml::from_str(&str).unwrap();

        return mapping
            .into_iter()
            .map(|(action, key_event)| (key_event, action))
            .collect();
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: keybinding("Ctrl + q"),
            navigate_down: SerializableKeyEvent::default(),
            navigate_up: SerializableKeyEvent::default(),
            navigate_left: SerializableKeyEvent::default(),
            navigate_right: SerializableKeyEvent::default(),
            navigate_home: SerializableKeyEvent::default(),
            navigate_end: SerializableKeyEvent::default(),
            focus_next: keybinding("Tab"),
            focus_previous: keybinding("Shift + Tab"),
        }
    }
}

fn keybinding(text: &str) -> SerializableKeyEvent {
    serde_json::from_str::<SerializableKeyEvent>(&format!("\"{}\"", text))
        .unwrap_or_else(|_| SerializableKeyEvent::default())
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default)]
    // Add your settings fields here
    pub search: SearchSettings,
    #[serde(default)]
    pub ui: UISettings,

    pub keybinds: KeyBindings,
}

impl Settings {
    pub fn new() -> Self {
        let path = get_settings_path();
        let config_file = path.join("settings.toml");

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

        let settings = match fs::read_to_string(_config_file.clone()) {
            Ok(content) => content,
            Err(_) => {
                log::warn!(
                    "Could not read settings file at {:?}, writing default settings",
                    _config_file
                );
                let default_settings = Settings::default();
                default_settings.write_default_settings(_config_file.clone());
                toml::to_string_pretty(&default_settings)
                    .expect("Could not serialize default settings")
            }
        };
        println!("settings file content: {:?}", settings);

        let structure: Settings =
            toml::from_str(&settings).expect("Could not deserialize settings");
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use KeyModifiers as Mod;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ftail::Ftail;
    use log::LevelFilter;
    use ratatui::widgets::ListState;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fmt::Write;

    #[test]
    fn test_write_default_settings() {
        let config_path = get_settings_path();
        let config_file = config_path.join("settings.toml");
        let settings = Settings::default();
        settings.write_default_settings(config_file.clone());

        log::info!("Settings struct: {:#?}", settings);
    }

    #[test]
    fn test_read_settings() {
        Ftail::new().console(LevelFilter::Trace).init().unwrap();
        let config_path = get_settings_path();
        let config_file = config_path.join("settings.toml");
        let raw_file =
            fs::read_to_string(&config_file).expect("Could not read settings file for testing");
        println!("Raw settings file content:\n{}", raw_file);
        let settings = Settings::read_settings(config_file);

        // Option A: deserialize to a generic JSON-like value to inspect nested structure
        let value = settings;
        log::info!("Settings struct: {:#?}", value);
    }

    #[test]
    fn toml_test_struct_serialise_vs_hashmap() {
        #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
        struct TestStruct {
            Quit: SerializableKeyEvent,
        }
        let mut hashmap = HashMap::new();
        hashmap.insert(
            Action::Quit,
            SerializableKeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            },
        );

        let test_struct = TestStruct {
            Quit: SerializableKeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            },
        };
        let toml_from_struct =
            toml::to_string_pretty(&test_struct).expect("Could not serialize test struct");
        let toml_from_hashmap =
            toml::to_string_pretty(&hashmap).expect("Could not serialize test hashmap");

        println!("TOML from struct:\n{}", toml_from_struct);
        println!("-------------------------");
        println!("TOML from hashmap:\n{}", toml_from_hashmap);

        let deserialized_struct: TestStruct =
            toml::from_str(&toml_from_struct).expect("Could not deserialize test struct");
        let deserialized_hashmap: HashMap<Action, SerializableKeyEvent> =
            toml::from_str(&toml_from_hashmap).expect("Could not deserialize test hashmap");

        let deserialized_struct_as_hashmap: HashMap<Action, SerializableKeyEvent> =
            toml::from_str(&toml_from_struct)
                .expect("Could not deserialize test struct to hashmap");
        let deserialized_hashmap_as_struct: TestStruct = toml::from_str(&toml_from_hashmap)
            .expect("Could not deserialize test hashmap to struct");

        println!("hashmap -> hashmap: {:#?}", deserialized_hashmap);
        println!("struct -> struct: {:#?}", deserialized_struct);
        println!("hashmap -> struct: {:#?}", deserialized_hashmap_as_struct);
        println!("struct -> hashmap: {:#?}", deserialized_struct_as_hashmap);
    }
}
//
