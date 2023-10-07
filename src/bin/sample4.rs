fn create_text() -> (&'static str, String, &'static str, &'static str) {
    let m1: &str = "Success!";
    let m2: String = "Information!".to_string();
    let m3: &str = "Failed!";
    let m4 = "Error!";
    (m1, m2, m3, m4)
}

fn regular(args: (&str, String, &str, &str)) {
    use consclr::Color; 

    // use buffer with size 32 bytes
    println!("{}", args.0.dgreen::<32>());
    println!("{}", args.1.dblue::<32>());
    println!("{}", args.2.dpurple::<32>());
    println!("{}", args.3.dred::<32>());
}

fn main() {
    regular(create_text());
}
