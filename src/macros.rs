/// print with color on stdout
/// # panic
/// will panic if formatter len > 256 bytes or
/// if text len > 256.
/// If you need to print text with len > 256,
/// consider using clr! macro instead
#[macro_export]
macro_rules! cprint {
    ($text:expr, $($s:expr),+) => {
        println!("{}", consclr::colorized!($text, $($s),+));
    };
}

/// print with color on stderr
/// # panic
/// will panic if formatter len > 256 bytes or
/// if text len > 256.
/// If you need to print text with len > 256,
/// consider using clr! macro instead
#[macro_export]
macro_rules! ceprint {
    ($text:expr, $($s:expr),+) => {
        eprintln!("{}", colorized!($text, $($s),+));
    };
}

/// helper macro for internal use
/// # panic
/// will panic if formatter len > 256 bytes or
/// if text len > 256.
/// If you need to print text with len > 256,
/// consider using clr! macro instead
#[macro_export]
macro_rules! colorized {
    ($text:expr, $($s:expr),+) => {{
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        $(bfr.append_str($s).unwrap();)+
        bfr.append_str($text).unwrap();
        bfr.append(consclr::helper::cl()).unwrap();
        bfr
    }};
}

/// convert a &str to color
/// If the argument is invalid, return a default color instead
/// (default color = white, default bg = black)
/// # Example
/// ```
/// use consclr::to_color;
///
/// let cgreen = to_color!("green");
/// ```
#[macro_export]
macro_rules! to_color {
    ($color:tt) => {{
        use consclr::colors::Formatter;
        match $color.to_lowercase().as_str() {
            "black" => consclr::colors::Regular::BLACK,
            "red" => consclr::colors::Regular::RED,
            "green" => consclr::colors::Regular::GREEN,
            "yellow" => consclr::colors::Regular::YELLOW,
            "blue" => consclr::colors::Regular::BLUE,
            "purple" => consclr::colors::Regular::PURPLE,
            "grblue" => consclr::colors::Regular::GRBLUE,
            "gray" => consclr::colors::Regular::GRAY,
            _ => consclr::colors::Regular::WHITE
        }
    }};
}

/// convert a &str to color bold
/// If the argument is invalid, return a default color instead
/// (default color = white, default bg = black)
/// # Example
/// ```
/// use consclr::to_colorb;
///
/// let cgreen = to_colorb!("green");
/// ```
#[macro_export]
macro_rules! to_colorb {
    ($color:tt) => {{
        use consclr::colors::Formatter;
        match $color.to_lowercase().as_str() {
            "black" => consclr::colors::Bold::BLACK,
            "red" => consclr::colors::Bold::RED,
            "green" => consclr::colors::Bold::GREEN,
            "yellow" => consclr::colors::Bold::YELLOW,
            "blue" => consclr::colors::Bold::BLUE,
            "purple" => consclr::colors::Bold::PURPLE,
            "grblue" => consclr::colors::Bold::GRBLUE,
            "gray" => consclr::colors::Bold::GRAY,
            _ => consclr::colors::Bold::WHITE
        }
    }};
}

/// Print without using buffer (Print with newline).
/// Because this macro does not store text
/// into the buffer, this macro can print very long text
/// # Panic
/// Although this macro did not store text on the buffer,
/// this macro still store formatting in the buffer.
/// If you use formatter with len > 256 bytes this
/// macro will panic
/// # Example
/// ```
/// use consclr::clr;
/// use consclr::colors::Regular;
/// use consclr::colors::Formatter;
/// 
/// let long_text = "Just pretend this is a very long text";
/// clr!(long_text, &Regular::RED.s());
/// ```
#[macro_export]
macro_rules! clr {
    ($text:expr, $s:tt) => {{
        use consclr::colors::Formatter;
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        let color = to_color!($s);
        bfr.append_str(&color.s()).unwrap();
        println!("{}{}{}", bfr, $text, consclr::helper::cl());
    }};
    ($text:expr, $($s:expr),+) => {{
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        $(bfr.append_str($s).unwrap();)+
        println!("{}{}{}", bfr, $text, consclr::helper::cl());
    }};
}

/// Print without using buffer (Print without newline).
/// Because this macro does not store text
/// into the buffer, this macro can print very long text
/// # Panic
/// Although this macro did not store text on the buffer,
/// this macro still store formatting in the buffer.
/// If you use formatter with len > 256 bytes this
/// macro will panic
/// # Example
/// ```
/// use consclr::clr2;
/// use consclr::colors::Regular;
/// use consclr::colors::Formatter;
/// 
/// let long_text = "Just pretend this is a very long text";
/// clr2!(long_text, &Regular::RED.s());
/// ```
#[macro_export]
macro_rules! clr2 {
    ($text:expr, $s:tt) => {{
        use consclr::colors::Formatter;
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        let color = to_color!($s);
        bfr.append_str(&color.s()).unwrap();
        print!("{}{}{}", bfr, $text, consclr::helper::cl());
    }};
    ($text:expr, $($s:expr),+) => {{
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        $(bfr.append_str($s).unwrap();)+
        print!("{}{}{}", bfr, $text, consclr::helper::cl());
    }};
}

/// Print char without using buffer.
/// Because this macro does not store text
/// into the buffer, this macro can print very long text.
/// You can specify an argument how many time you want
/// to repeat printing the char.
/// # Panic
/// Although this macro did not store text on the buffer,
/// this macro still store formatting in the buffer.
/// If you use formatter with len > 256 bytes this
/// macro will panic
/// # Example
/// ```
/// use consclr::pchar;
/// use consclr::colors::Regular;
/// use consclr::colors::Formatter;
/// 
/// let long_text = "Just pretend this is a very long text";
/// // this code will print "*" 100 times with regular red color
/// pchar!("*", 100, &Regular::RED.s());
/// ```
#[macro_export]
macro_rules! pchar {
    ($char:expr, $len:tt, $($s:expr),+) => {{
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        $(bfr.append_str($s).unwrap();)+
        println!("{}{}{}", bfr, $char.repeat($len), consclr::helper::cl());
    }};
}
