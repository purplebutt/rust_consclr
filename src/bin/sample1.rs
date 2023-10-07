fn create_text() -> (&'static str, String, &'static str, &'static str) {
    let m1: &str = "Success!";
    let m2: String = "Information!".to_string();
    let m3: &str = "Failed!";
    let m4 = "Error!";
    (m1, m2, m3, m4)
}

fn regular(args: (&str, String, &str, &str)) {
    use consclr::Color256;

    println!("{}", args.0.green());
    println!("{}", args.1.blue());
    println!("{}", args.2.purple());
    println!("{}", args.3.red());
}

fn underline(args: (&str, String, &str, &str)) {
    use consclr::ColorUl256;

    println!("{}", args.0.ugreen());
    println!("{}", args.1.ublue());
    println!("{}", args.2.upurple());
    println!("{}", args.3.ured());
}

fn background(args: (&str, String, &str, &str)) {
    use consclr::ColorBg256;

    println!("{}", args.0.bggreen());
    println!("{}", args.1.bgblue());
    println!("{}", args.2.bgpurple());
    println!("{}", args.3.bgred());
}

fn bold(args: (&str, String, &str, &str)) {
    use consclr::ColorBold256;

    println!("{}", args.0.greenb());
    println!("{}", args.1.blueb());
    println!("{}", args.2.purpleb());
    println!("{}", args.3.redb());
}

fn main() {
    regular(create_text());
    background(create_text());
    underline(create_text());
    bold(create_text());
}

