use std::collections::HashMap;


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
    pub pre_query: String,      // text before the query input
    pub caret: String,          // caret character
    pub caret_blink_rate: u64,  // in ms
    pub caret_visible: bool,    // if disabled, remove blinking, caret, and care movement    // if true, search as you type
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
    max_results: usize, // maximum number of results to display
    show_scores: bool,  // whether to show scores next to results
}
impl Default for UIResultsSettings {
    fn default() -> Self {
        Self {
            max_results: 20,
            show_scores: true,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UITooltipSettings {
    pub enabled: bool,          // whether tooltips are enabled
    pub max_width: usize,      // maximum width of tooltip
    pub max_height: usize,     // maximum height of tooltip
    pub delay: u64,            // delay before showing tooltip in ms
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
    sections: Vec<String>, // order of layout sections
}
impl Default for UILayoutSettings {
    fn default() -> Self {
        Self {
            sections: vec![
                "search".into(),
                "results".into(),
                "tooltip".into(),
            ],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeSettings {
    pub background: String,
    pub foreground: String,
    pub highlight: String,
    pub accent: String,
    pub caret: String,
    pub border: String,

    pub search_background: String,
    pub search_foreground: String,
    pub search_accent: String,
    pub search_caret: String,
    pub search_border: String,
}
impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            background: "Black".into(),
            foreground: "White".into(),
            highlight: "Yellow".into(),
            accent: "Cyan".into(),
            caret: "White".into(),
            border: "Blue".into(),

            search_background: "DarkGray".into(),
            search_foreground: "White".into(),
            search_accent: "Cyan".into(),
            search_caret: "White".into(),
            search_border: "Blue".into(),
        }
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
