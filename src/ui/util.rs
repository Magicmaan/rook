#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
