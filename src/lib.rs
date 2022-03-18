
#[allow(dead_code)]
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
    DarkGray,
    LightRed,
    LightGreen,
    Yellow,
    LightBlue,
    LightPurple,
    LightCyan,
    White,
}

#[allow(dead_code)]
impl Color {
    fn to_foreground(&self) -> &str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Brown => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Purple => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::LightGray => "\x1b[37m",
            Color::DarkGray => "\x1b[1;30m",
            Color::LightRed => "\x1b[1;31m",
            Color::LightGreen => "\x1b[1;32m",
            Color::Yellow => "\x1b[1;33m",
            Color::LightBlue => "\x1b[1;34m",
            Color::LightPurple => "\x1b[1;35m",
            Color::LightCyan => "\x1b[1;36m",
            Color::White => "\x1b[1;37m",
        }
    }

    fn to_background(&self) -> &str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Black => "\x1b[0;40m",
            Color::Red => "\x1b[0;41m",
            Color::Green => "\x1b[0;42m",
            Color::Brown => "\x1b[0;43m",
            Color::Blue => "\x1b[0;44m",
            Color::Purple => "\x1b[0;45m",
            Color::Cyan => "\x1b[0;46m",
            Color::LightGray => "\x1b[0;47m",
            Color::DarkGray => "\x1b[1;40m",
            Color::LightRed => "\x1b[1;41m",
            Color::LightGreen => "\x1b[1;42m",
            Color::Yellow => "\x1b[1;43m",
            Color::LightBlue => "\x1b[1;44m",
            Color::LightPurple => "\x1b[1;45m",
            Color::LightCyan => "\x1b[1;46m",
            Color::White => "\x1b[1;47m",
        }
    }
}

/// A style is a 
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

    pub fn with_foreground(self, fg: Color) -> Self {
        Style { fg: Some(fg), ..self }
    }

    pub fn without_foreground(self) -> Self {
        Style { fg: None, ..self }
    }

    pub fn with_background(self, bg: Color) -> Self {
        Style { bg: Some(bg), ..self }
    }


    pub fn fmt(&self, text: &str) -> String {
        format!(
            "{}{}{}{}",
            "", "",
            // self.bg.to_background(),
            // self.fg.to_foreground(),
            text,
            Color::Reset.to_foreground()
        )
    }
}
