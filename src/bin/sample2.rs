use consclr::helper::cl;
use consclr::colors::{Regular, Background, Special};


fn main() {
    let bgred = Background::RED;
    let blue = Regular::BLUE;

    let bold = Special::BOLD;
    let blink = Special::BLINK;
    let underline = Special::UNDERLINE;
    let italic = Special::ITALIC;

    let format = format!("{}{}{}{}{}", bgred, bold, blink, underline, italic);
    let blue_text = format!("{}Hello World", blue);

    let formatted = format!("{}{}{}", format, blue_text, cl());

    println!("{}", formatted);
}
