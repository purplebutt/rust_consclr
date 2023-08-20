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

/// Print without using buffer.
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
    ($text:expr, $($s:expr),+) => {{
        let mut bfr = <cbfr::prelude::BFRDYN>::new();
        $(bfr.append_str($s).unwrap();)+
        println!("{}{}{}", bfr, $text, consclr::helper::cl());
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
