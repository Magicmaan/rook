use ratatui::{style::Color, widgets::BorderType};
use serde::{Deserialize, Deserializer, Serializer};

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
        None => serializer.serialize_str(""),
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
        Some(s) => match s.to_lowercase().as_str() {
            "plain" => Ok(Some(BorderType::Plain)),
            "rounded" => Ok(Some(BorderType::Rounded)),
            "double" => Ok(Some(BorderType::Double)),
            "thick" => Ok(Some(BorderType::Thick)),
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
    log::info!("Deserializing color: {}", s);
    match s.to_lowercase().as_str() {
        "reset" => Ok(Color::Reset),
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "gray" => Ok(Color::Gray),
        "darkgray" => Ok(Color::DarkGray),
        "lightred" => Ok(Color::LightRed),
        "lightgreen" => Ok(Color::LightGreen),
        "lightyellow" => Ok(Color::LightYellow),
        "lightblue" => Ok(Color::LightBlue),
        "lightmagenta" => Ok(Color::LightMagenta),
        "lightcyan" => Ok(Color::LightCyan),
        "white" => Ok(Color::White),
        // rgb color in format "r,g,b"
        s if s.chars().next().unwrap_or('a').is_numeric() => {
            // Indexed color
            if s.contains(",") {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() == 3 {
                    let r = parts[0].parse().map_err(serde::de::Error::custom)?;
                    let g = parts[1].parse().map_err(serde::de::Error::custom)?;
                    let b = parts[2].parse().map_err(serde::de::Error::custom)?;
                    return Ok(Color::Rgb(r, g, b));
                } else {
                    return Err(serde::de::Error::custom("Invalid RGB format"));
                }
            } else {
                return Ok(Color::Red); // fallback for single number
            }
            // Ok(Color::Indexed(index))
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
        None => serializer.serialize_str(""),
    }
}

pub fn deserialize_optional_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    log::info!("Deserializing optional color: {:?}", opt);
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
            return serializer.serialize_str(&format!("{},{},{}", r, g, b));
        }
        Color::Indexed(i) => return serializer.serialize_str(&format!("Indexed({})", i)),
    };
    serializer.serialize_str(s)
}

pub fn serialize_alignment<S>(
    alignment: &ratatui::layout::Alignment,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = match alignment {
        ratatui::layout::Alignment::Left => "left",
        ratatui::layout::Alignment::Center => "center",
        ratatui::layout::Alignment::Right => "right",
    };
    serializer.serialize_str(s)
}

pub fn deserialize_alignment<'de, D>(
    deserializer: D,
) -> Result<ratatui::layout::Alignment, D::Error>
where
    D: Deserializer<'de>,
{
    log::info!("Deserializing alignment...");

    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "left" => {
            log::info!(
                "Deserialized alignment from string: {} to {:?}",
                s,
                ratatui::layout::Alignment::Left
            );
            Ok(ratatui::layout::Alignment::Left)
        }
        "center" => {
            log::info!(
                "Deserialized alignment from string: {} to {:?}",
                s,
                ratatui::layout::Alignment::Center
            );
            Ok(ratatui::layout::Alignment::Center)
        }
        "right" => {
            log::info!(
                "Deserialized alignment from string: {} to {:?}",
                s,
                ratatui::layout::Alignment::Right
            );
            Ok(ratatui::layout::Alignment::Right)
        }
        _ => {
            log::info!(
                "Deserialized UNKNOWN alignment from string: {} to {:?}",
                s,
                ratatui::layout::Alignment::Left
            );
            Ok(ratatui::layout::Alignment::Left)
        }
    }
}
