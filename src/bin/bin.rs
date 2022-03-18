use ansirs::{Style, Color};


fn main() {
    println!("Hello, world!");
    let empty = Style::new();
    // println!("empty = {:?}", empty);
    
    let red = empty
        .with_foreground(Color::Red);
    // println!("empty = {:?}", empty);
    // println!("red   = {:?}", red);

    let blue = Style::new()
        .with_background(Color::Blue);

    let both = Style::new()
        .with_foreground(Color::Red)
        .with_background(Color::Blue);
    // println!("both = {:?}", both);

    println!("Start {} {} {} End", red.fmt("Red-FG"), blue.fmt("Blue-BG"), both.fmt("Both"));

    println!("Start  \x1b[0;31mOne\x1b[0m  \x1b[0;44mTwo\x1b[0m  \x1b[1;44m \x1b[1;31m \x1b[5m Three \x1b[0m  End");

    println!("Start \x1b[1;4;31;42mBlink!\x1b[0m End");

}
