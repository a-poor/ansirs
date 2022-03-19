use ansirs::{Color, Style};

fn main() {
    println!("Hello, world!");
    let empty = Style::new();

    let red = empty
        .with_foreground(Color::Red);

    let blue = Style::new()
        .with_background(Color::Blue);

    let both = blue
        .with_foreground(Color::Red)
        .with_background(Color::Blue)
        .with_bold(true)
        .with_hidden(true);
    // println!("both = {:?}", both);

    println!(
        "Start {} {} {} End",
        red.fmt("Red-FG".to_string()),
        blue.fmt("Blue-BG".to_string()),
        both.fmt("Both".to_string())
    );

    println!("");
    let style = Style::new()
        .with_foreground(Color::Red)
        .with_background(Color::Blue)
        .with_bold(true)
        .with_width(30);
    let text = r#"Hello, World!
Hello, from Rust!

..."#;
    let res = style.fmt(text.to_string());
    println!("{}", res);
}
