use ansirs::{Color, Style};

fn main() {
    let style = Style::new()
        .with_foreground(Color::Red)
        .with_background(Color::Blue)
        .with_bold(true)
        .with_height(10)
        .with_width(30);
    let text = r#"Hello, World!
Hello, from Rust!

..."#;
    let res = style.fmt(text.to_string());
    println!("{}", res);
}
