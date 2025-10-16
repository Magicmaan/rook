use std::collections::HashMap;

use config::Config;
use dirs::config_dir;
use ratatui::style::Style;
use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

use crate::model::ui::UISection;
use crate::ui::util::IconMode;

// helper functions for serializing/deserializing ratatui types
// stupid Color and BorderType don't implement Serialize/Deserialize >:(
// used in settings structs


// deserialize BorderType from string
pub fn deserialize_border_type<'de, D>(deserializer: D) -> Result<BorderType, D::Error>
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

// serialize Option<BorderType>
pub fn serialize_optional_border_type<S>(
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

pub fn deserialize_optional_border_type<'de, D>(
    deserializer: D,
) -> Result<Option<BorderType>, D::Error>
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

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
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

pub fn serialize_optional_color<S>(color: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match color {
        Some(c) => serialize_color(c, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_optional_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
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

pub fn serialize_border_type<S>(border_type: &BorderType, serializer: S) -> Result<S::Ok, S::Error>
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

pub fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
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
