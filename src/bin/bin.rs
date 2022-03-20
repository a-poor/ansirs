use ansirs::{Color, Style};

fn main() {
    let style = Style::new()
        .with_foreground(Color::Red)
        .with_background(Color::Blue)
        .with_bold(true)
        .with_height(10)
        .with_width(30)
        .with_max_width(35)
        ;
        
    let text = r#"Hello, World!
Hello, from Rust!


This is a pretty long sentence. Was it truncated?

..."#;
    let res = style.fmt(text.to_string());
    println!("{}", res);
}
