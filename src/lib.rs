const ANSI_ESCAPE_PREFIX: &str = "\x1b[";
const ANSI_ESCAPE_SUFFIX: &str = "m";
const ANSI_RESET_STYLE_CODE: &str = "\x1b[0m";

const ANSI_FOREGROUND_PREFIX: &str = "3";
const ANSI_BACKGROUND_PREFIX: &str = "4";

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Brown,
    Blue,
    Purple,
    Cyan,
    LightGray,
    // DarkGray,
    // LightRed,
    // LightGreen,
    // Yellow,
    // LightBlue,
    // LightPurple,
    // LightCyan,
    // White,
}

impl Color {
    fn get_ansi_num(self) -> &'static str {
        match self {
            Black => "0",
            Red => "1",
            Green => "2",
            Brown => "3",
            Blue => "4",
            Purple => "5",
            Cyan => "6",
            LightGray => "7",
            Reset => "9",
        }
    }

    fn get_ansi_fg(self) -> String {
        format!(
            "{}{}{}",
            ANSI_ESCAPE_PREFIX,
            self.get_ansi_num(),
            ANSI_ESCAPE_SUFFIX
        )
    }
}

/// Style stores terminal formating information.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Style {
    /// Foreground color
    fg: Option<Color>,

    /// Background color
    bg: Option<Color>,

    /// Is the text bold?
    bold: Option<bool>,

    /// Is the text italic?
    italic: Option<bool>,

    /// Is the text underlined?
    underline: Option<bool>,

    /// Is the text strikethrough?
    inverted: Option<bool>,

    /// Is the text hidden?
    hidden: Option<bool>,

    /// Is the text crossed out?
    strikethrough: Option<bool>,
}

#[allow(dead_code)]
impl Style {
    /// Create a new, empty Style.
    pub fn new() -> Self {
        Style {
            fg: None,
            bg: None,
            bold: None,
            underline: None,
            inverted: None,
            hidden: None,
            strikethrough: None,
            italic: None,
        }
    }

    /// Set the foreground color.
    pub fn with_foreground(self, fg: Color) -> Self {
        Style {
            fg: Some(fg),
            ..self
        }
    }

    /// Remove the foreground color.
    pub fn with_no_foreground(self) -> Self {
        Style { fg: None, ..self }
    }

    /// Set the background color.
    pub fn with_background(self, bg: Color) -> Self {
        Style {
            bg: Some(bg),
            ..self
        }
    }

    /// Remove the background color.
    pub fn with_no_background(self) -> Self {
        Style { fg: None, ..self }
    }

    /// Format the given text with this style.
    pub fn fmt(self, text: &str) -> String {
        format!(
            "{}{}{}{}",
            "",
            "",
            // self.bg.to_background(),
            // self.fg.to_foreground(),
            text,
            Color::Reset.to_foreground()
        )
    }

    fn get_fg(self) -> String {
        match self.fg {
            Some(fg) => match fg {
                Color::Reset => format!(""),
                Color::Black => format!(""),
                Color::Red => format!(""),
                Color::Green => format!(""),
                Color::Brown => format!(""),
                Color::Blue => format!(""),
                Color::Purple => format!(""),
                Color::Cyan => format!(""),
            },
            None => "".to_string(),
        }
    }
    fn get_bg(self) -> String {
        match self.bg {
            Some(bg) => "",
            None => "".to_string(),
        }
    }
    fn get_bold(self) -> String {
        match self.bold {
            Some(bold) => "",
            None => "",
        }
    }
    fn get_underline(self) -> String {
        match self.underline {
            Some(underline) => "",
            None => "".to_string(),
        }
    }
    fn get_inverted(self) -> String {
        match self.inverted {
            Some(inverted) => "",
            None => "".to_string(),
        }
    }
    fn get_hidden(self) -> String {
        match self.hidden {
            Some(hidden) => "",
            None => "".to_string(),
        }
    }
    fn get_strikethrough(self) -> String {
        match self.strikethrough {
            Some(strikethrough) => "",
            None => "".to_string(),
        }
    }
    fn get_italic(self) -> String {
        match self.italic {
            Some(italic) => "",
            None => "".to_string(),
        }
    }
}
