use std::cmp;

const ANSI_ESCAPE_PREFIX: &str = "\x1b[";
const ANSI_ESCAPE_SUFFIX: &str = "m";
const ANSI_RESET_STYLE_CODE: &str = "\x1b[0m";

const ANSI_FOREGROUND_PREFIX: &str = "3";
const ANSI_BACKGROUND_PREFIX: &str = "4";

const ANSI_BOLD_STYLE_CODE: &str = "1";
const ANSI_RESET_BOLD_STYLE_CODE: &str = "22";

// const ANSI_FAINT_STYLE_CODE: &str = "2";

const ANSI_ITALIC_STYLE_CODE: &str = "3";
const ANSI_RESET_ITALIC_STYLE_CODE: &str = "23";

const ANSI_UNDERLINE_STYLE_CODE: &str = "4";
const ANSI_RESET_UNDERLINE_STYLE_CODE: &str = "24";
// const ANSI_DOUBLE_UNDERLINE_STYLE_CODE: &str = "21";

// const ANSI_SLOW_BLINK_STYLE_CODE: &str = "5";
// const ANSI_RESET_SLOW_BLINK_STYLE_CODE: &str = "25";
// const ANSI_RAPID_BLINK_STYLE_CODE: &str = "6";
// const ANSI_RESET_RAPID_BLINK_STYLE_CODE: &str = "26";

const ANSI_INVERT_STYLE_CODE: &str = "7";
const ANSI_RESET_INVERT_STYLE_CODE: &str = "27";

const ANSI_HIDE_STYLE_CODE: &str = "8";
const ANSI_RESET_HIDE_STYLE_CODE: &str = "28";

const ANSI_STRIKETHROUGH_STYLE_CODE: &str = "9";
const ANSI_RESET_STRIKETHROUGH_STYLE_CODE: &str = "29";


#[derive(Debug, Copy, Clone)]
pub enum Align {
    Start,
    Middle,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Color {
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

impl Color {
    fn get_ansi_color_num(self) -> &'static str {
        match self {
            Color::Black     | Color::DarkGray    => "0",
            Color::Red       | Color::LightRed    => "1",
            Color::Green     | Color::LightGreen  => "2",
            Color::Brown     | Color::Yellow      => "3",
            Color::Blue      | Color::LightBlue   => "4",
            Color::Purple    | Color::LightPurple => "5",
            Color::Cyan      | Color::LightCyan   => "6",
            Color::LightGray | Color::White       => "7",
        }
    }

    fn is_bold(self) -> bool {
        match self {
            Color::Black | Color::Red | Color::Green | Color::Brown | Color::Blue | Color::Purple | Color::Cyan | Color::LightGray => false,
            Color::DarkGray | Color::LightRed | Color::LightGreen | Color::Yellow | Color::LightBlue | Color::LightPurple | Color::LightCyan | Color::White => true,
        }
    }

    fn as_foreground(self) -> String {
        let num = self.get_ansi_color_num();
        let prefix = if self.is_bold() { "1" } else { "22" };
        if self.is_bold() {
            format!("{};{}{}", prefix, ANSI_FOREGROUND_PREFIX, num)
        } else {
            format!("{};{}{}", prefix, ANSI_FOREGROUND_PREFIX, num)
        }
    }

    fn as_background(self) -> String {
        let num = self.get_ansi_color_num();
        let prefix = if self.is_bold() { "1" } else { "22" };
        if self.is_bold() {
            format!("{};{}{}", prefix, ANSI_BACKGROUND_PREFIX, num)
        } else {
            format!("{};{}{}", prefix, ANSI_BACKGROUND_PREFIX, num)
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Spacing {
    top: usize, 
    right: usize, 
    left: usize, 
    bottom: usize
}

/// Style stores terminal formating information.
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
    /// 
    /// TODO - Add double underline...
    underline: Option<bool>,

    /// Is the text strikethrough?
    inverted: Option<bool>,

    /// Is the text hidden?
    hidden: Option<bool>,

    /// Is the text crossed out?
    strikethrough: Option<bool>,

    min_width: Option<usize>,
    min_height: Option<usize>,

    max_width: Option<usize>,
    max_height: Option<usize>,

    halign: Align,
    valign: Align,

    padding: Spacing,
    margin: Spacing,
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
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            halign: Align::Start,
            valign: Align::Start,
            padding: Spacing::default(),
            margin: Spacing::default(),
        }
    }

    /// Return a new Style based on the current Style, but with
    /// the specified foreground color.
    pub fn with_foreground(self, fg: Color) -> Self {
        Style {
            fg: Some(fg),
            ..self
        }
    }

    /// Return a new Style based on the current Style, but with
    /// the foreground cleared.
    pub fn clear_foreground(self) -> Self {
        Style { fg: None, ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the specified background color.
    pub fn with_background(self, bg: Color) -> Self {
        Style {
            bg: Some(bg),
            ..self
        }
    }

    /// Return a new Style based on the current Style, but with
    /// the background color cleared.
    pub fn clear_background(self) -> Self {
        Style { fg: None, ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the specified boldness.
    pub fn with_bold(self, is_bold: bool) -> Self {
        Style { bold: Some(is_bold), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the bold style cleared.
    pub fn clear_bold(self) -> Self {
        Style { bold: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified underlined-ness.
    pub fn with_underline(self, is_underlined: bool) -> Self {
        Style { underline: Some(is_underlined), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the underline style cleared.
    pub fn clear_underline(self) -> Self {
        Style { underline: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified color-inversion.
    pub fn with_inverted(self, is_inverted: bool) -> Self {
        Style { inverted: Some(is_inverted), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the color-inversion style cleared.
    pub fn clear_inverted(self) -> Self {
        Style { inverted: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified hidden-ness.
    pub fn with_hidden(self, is_hidden: bool) -> Self {
        Style { hidden: Some(is_hidden), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the hidden style cleared.
    pub fn clear_hidden(self) -> Self {
        Style { hidden: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified strikethrough-ness.
    pub fn with_strikethrough(self, is_strikenthrough: bool) -> Self {
        Style { strikethrough: Some(is_strikenthrough), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the strikethrough style cleared.
    pub fn clear_strikethrough(self) -> Self {
        Style { strikethrough: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified italic-ness.
    pub fn with_italic(self, is_italic: bool) -> Self {
        Style { italic: Some(is_italic), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the italics style cleared.
    pub fn clear_italic(self) -> Self {
        Style { italic: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified minimum line width.
    pub fn with_width(self, width: usize) -> Self {
        Style { min_width: Some(width), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the minimum height cleared.
    pub fn clear_width(self) -> Self {
        Style { min_width: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified minimum height.
    pub fn with_height(self, height: usize) -> Self {
        Style { min_height: Some(height), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the minimum height cleared.
    pub fn clear_height(self) -> Self {
        Style { min_height: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified maximum line width.
    pub fn with_max_width(self, width: usize) -> Self {
        Style { max_width: Some(width), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the maximum height cleared.
    pub fn clear_max_width(self) -> Self {
        Style { max_width: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified maximum height.
    pub fn with_max_height(self, height: usize) -> Self {
        Style { max_height: Some(height), ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the maximum height cleared.
    pub fn clear_max_height(self) -> Self {
        Style { max_height: None, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified maximum height.
    pub fn with_haligh(self, align: Align) -> Self {
        Style { halign: align, ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the maximum height cleared.
    pub fn clear_halign(self) -> Self {
        Style { halign: Align::Start, ..self}
    }

    /// Return a new Style based on the current Style, but with
    /// the specified maximum height.
    pub fn with_valigh(self, align: Align) -> Self {
        Style { valign: align, ..self }
    }

    /// Return a new Style based on the current Style, but with
    /// the maximum height cleared.
    pub fn clear_valign(self) -> Self {
        Style { valign: Align::Start, ..self}
    }

    /// Format the given text with this style.
    pub fn fmt(self, text: String) -> String {
        // Create a vector to store the ANSI codes.
        let mut ansi_codes: Vec<String> = Vec::new();

        // Add the fore-/back-ground colors
        if let Some(fg) = self.fg {
            ansi_codes.push(fg.as_foreground());
        }
        if let Some(bg) = self.bg {
            ansi_codes.push(bg.as_background());
        }

        // Add bold/italic/underlined
        if let Some(bold) = self.bold {
            if bold {
                ansi_codes.push(ANSI_BOLD_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_BOLD_STYLE_CODE.to_string());
            }
        }
        if let Some(italic) = self.italic {
            if italic {
                ansi_codes.push(ANSI_ITALIC_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_ITALIC_STYLE_CODE.to_string());
            }
        }
        if let Some(underline) = self.underline {
            if underline {
                ansi_codes.push(ANSI_UNDERLINE_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_UNDERLINE_STYLE_CODE.to_string());
            }
        }
        
        // Add inverted/hidden/strikethrough
        if let Some(invert) = self.inverted {
            if invert {
                ansi_codes.push(ANSI_INVERT_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_INVERT_STYLE_CODE.to_string());
            }
        }
        if let Some(hiden) = self.hidden {
            if hiden {
                ansi_codes.push(ANSI_HIDE_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_HIDE_STYLE_CODE.to_string());
            }
        }
        if let Some(strikethrough) = self.strikethrough {
            if strikethrough {
                ansi_codes.push(ANSI_STRIKETHROUGH_STYLE_CODE.to_string());
            } else {
                ansi_codes.push(ANSI_RESET_STRIKETHROUGH_STYLE_CODE.to_string());
            }
        }

        // Join the ANSI codes with semi-colons
        let ansi_codes = ansi_codes.join(";");

        let mut formatted = self.fmt_height(text);
        formatted = self.fmt_width(formatted);

        // Format the string and return it...
        formatted.split("\n").map(|line| {
            format!(
                "{}{}{}{}{}",
                ANSI_ESCAPE_PREFIX,
                ansi_codes,
                ANSI_ESCAPE_SUFFIX,
                line,
                ANSI_RESET_STYLE_CODE
            )
        }).collect::<Vec<String>>().join("\n")
    }


    fn fmt_width(self, text: String) -> String {
        // Calculate line widths
        let mut width = text.split("\n").map(|line| line.len()).max().unwrap_or(0);
        if let Some(min_width) = self.min_width {
            width = cmp::max(width, min_width);
        }
        if let Some(max_width) = self.max_width {
            width = cmp::min(width, max_width);
        }

        text.split("\n").map(|line| {
            let mut line = line.to_string();
            if width > line.len() {
                let diff = width - line.len();
                match self.halign {
                    Align::Start => {
                        line.push_str(&" ".repeat(diff));
                    },
                    Align::Middle => {
                        let left_pad = diff / 2;
                        let right_pad = diff - left_pad;
                        line.insert_str(0, &" ".repeat(left_pad));
                        line.push_str(&" ".repeat(right_pad));
                    },
                    Align::End => {
                        line.insert_str(0, &" ".repeat(diff));
                    },
                }
            }
            if width < line.len() {
                line.truncate(width);
            }
            line
        }).collect::<Vec<String>>().join("\n")
    }

    fn fmt_height(self, text: String) -> String {
        // Make a copy of the text
        let mut new_text = text;

        // Add top/bottom margin and padding, if specified
        
        // Add newlines to pad to minimum width, if necessary
        if let Some(min_height) = self.min_height {
            if new_text.split('\n').count() < min_height {
                let diff = min_height - new_text.split('\n').count();
                match self.valign {
                    Align::Start => {
                        new_text.push_str(&"\n".repeat(diff));
                    },
                    Align::Middle => {
                        let left_pad = diff / 2;
                        let right_pad = diff - left_pad;
                        new_text.insert_str(0, &"\n".repeat(left_pad));
                        new_text.push_str(&"\n".repeat(right_pad));
                    },
                    Align::End => {
                        new_text.insert_str(0, &"\n".repeat(diff));
                    },
                }
            }
        }
        if let Some(max_height) = self.max_height {
            if new_text.split('\n').count() > max_height {
                new_text.truncate(new_text.rfind('\n').unwrap() + 1);
            }
        }
        new_text
    }

}

struct Text {    
    min_width: Option<usize>,
    max_width: Option<usize>,
    min_height: Option<usize>,
    max_height: Option<usize>,

    halign: Align,
    valign: Align,

    margin: Spacing,
    padding: Spacing,
}

impl Text {
    fn new(text: String) -> Text {
        Text {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            halign: Align::Start,
            valign: Align::Start,
            margin: Spacing::default(),
            padding: Spacing::default(),
        }
    }

    /// Format the supplied text with the specified formatting options
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text to format
    /// * `formatter` - A function that takes a string and returns a formatted string
    /// 
    fn fmt(self, text: String, formatter: &dyn Fn(String) -> String) -> String {
        // Get the supplied text's width and height
        let height = text.split('\n').count();
        let width = text.split('\n').map(|line| line.len()).max().unwrap_or(0);

        // Create a container to hold the resulting text
        let mut lines = text.split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        // lines = lines.iter().map(|line| {
        //     // Format the line
        //     let line = formatter(line.to_string());

        //     // Return the line
        //     line
        // }).collect::<Vec<String>>();

        // Return the formatted text
        lines.join("\n")
    }
}

// TODO: Rewrite the min/max width/height (w/ padding & margin) formatter.
// 
// - min-width should include padding but not margin
// - max-width should include all spaces but shouldn't affect the styling
// 
// 
// - style and format the text as a vector of strings
// - start with the styled text (line text + min-width/height + padding), but not styled yet
// - ...
// 

