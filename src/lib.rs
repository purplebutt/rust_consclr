//!
//! A library to colorized your terminal
//! This library provides traits that give extension function
//! for &str, &String and String and also macro.
//!
//! # 1. Traits
//! There are two types of traits that available, the fixed 256 traits
//! and traits with generic type to work with dynamic buffer size.
//!
//! # 256 examples
//! ```
//! 
//! fn create_text() -> (&'static str, String, &'static str, &'static str) {
//!     let m1: &str = "Success!";
//!     let m2: String = "Information!".to_string();
//!     let m3: &str = "Failed!";
//!     let m4 = "Error!";
//!     (m1, m2, m3, m4)
//! }
//! 
//! fn regular(args: (&str, String, &str, &str)) {
//!     use consclr::Color256;
//! 
//!     println!("{}", args.0.green());
//!     println!("{}", args.1.blue());
//!     println!("{}", args.2.purple());
//!     println!("{}", args.3.red());
//! }
//!
//! fn underline(args: (&str, String, &str, &str)) {
//!     use consclr::ColorUl256;
//! 
//!     println!("{}", args.0.ugreen());
//!     println!("{}", args.1.ublue());
//!     println!("{}", args.2.upurple());
//!     println!("{}", args.3.ured());
//! }
//! 
//! fn background(args: (&str, String, &str, &str)) {
//!     use consclr::ColorBg256;
//! 
//!     println!("{}", args.0.bggreen());
//!     println!("{}", args.1.bgblue());
//!     println!("{}", args.2.bgpurple());
//!     println!("{}", args.3.bgred());
//! }
//! 
//! fn bold(args: (&str, String, &str, &str)) {
//!     use consclr::ColorBold256;
//! 
//!     println!("{}", args.0.greenb());
//!     println!("{}", args.1.blueb());
//!     println!("{}", args.2.purpleb());
//!     println!("{}", args.3.redb());
//! }
//! 
//! fn main() {
//!     regular(create_text());
//!     underline(create_text());
//!     background(create_text());
//!     bold(create_text());
//! }
//! ```
//!
//! # dynamic generic type
//! 
//! ```
//! fn create_text() -> (&'static str, String, &'static str, &'static str) {
//!     let m1: &str = "Success!";
//!     let m2: String = "Information!".to_string();
//!     let m3: &str = "Failed!";
//!     let m4 = "Error!";
//!     (m1, m2, m3, m4)
//! }
//! 
//! fn regular(args: (&str, String, &str, &str)) {
//!     use consclr::Color; 
//! 
//!     // use buffer with size 32 bytes
//!     println!("{}", args.0.dgreen::<32>());
//!     println!("{}", args.1.dblue::<32>());
//!     println!("{}", args.2.dpurple::<32>());
//!     println!("{}", args.3.dred::<32>());
//! }
//! 
//! fn main() {
//!     regular(create_text());
//! }
//! ```

pub mod colors; pub mod helper; pub mod macros; pub mod prelude;

use cbfr::prelude::BFRDYN;
use crate::colors::{
    Formatter, 
    Regular, 
    Background, 
    Underline,
    Special, 
    Bold
};

#[doc(hidden)]
///
/// helper function for internal use
/// this function simplified styling process
///
fn stylify(text: &str, color: impl Formatter) -> BFRDYN {
    let mut b: BFRDYN = color.to_string().as_str().into();
    b.append_str(text).unwrap();
    b.append_str(Special::DEF.to_string().as_str()).unwrap();
    b
}

///
/// A trait that contain methods to format self with regular color
/// once this trait imported, &str, &String and String will
/// have extension method to format it with color on terminal
/// # Example
/// ```
/// use consclr::{Color256};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.red());
/// ```
pub trait Color256: ToString {
    fn black(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::BLACK)
    }
    fn red(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::RED)
    }
    fn green(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::GREEN)
    }
    fn yellow(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::YELLOW)
    }
    fn blue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::BLUE)
    }
    fn purple(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::PURPLE)
    }
    fn grblue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::GRBLUE)
    }
    fn gray(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::GRAY)
    }
    fn white(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Regular::WHITE)
    }
}

///
/// A trait that contain methods to format self with color and Bold
/// once this trait imported, &str, &String and String will
/// have extension method to format it with color and Bold on terminal
/// # Example
/// ```
/// use consclr::{ColorBold256};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.redb());
/// ```
pub trait ColorBold256: ToString {
    fn blackb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::BLACK)
    }
    fn redb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::RED)
    }
    fn greenb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::GREEN)
    }
    fn yellowb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::YELLOW)
    }
    fn blueb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::BLUE)
    }
    fn purpleb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::PURPLE)
    }
    fn grblueb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::GRBLUE)
    }
    fn grayb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::GRAY)
    }
    fn whiteb(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Bold::WHITE)
    }
}

///
/// A trait that contain methods to format self with background color
/// once this trait imported, &str, &String and String will
/// have extension method to format it with bg color on terminal
/// # Example
/// ```
/// use consclr::{ColorBg256};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.bgred());
/// ```
pub trait ColorBg256: ToString {
    fn bgblack(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::BLACK)
    }
    fn bgred(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::RED)
    }
    fn bggreen(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::GREEN)
    }
    fn bgyellow(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::YELLOW)
    }
    fn bgblue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::BLUE)
    }
    fn bgpurple(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::PURPLE)
    }
    fn bggrblue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::GRBLUE)
    }
    fn bggray(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::GRAY)
    }
    fn bgwhite(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Background::WHITE)
    }
}

///
/// A trait that contain methods to format self with background color
/// once this trait imported, &str, &String and String will
/// have extension method to format it with bg color on terminal
/// # Example
/// ```
/// use consclr::ColorUl256;
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.ured());
/// ```
pub trait ColorUl256: ToString {
    fn ublack(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::BLACK)
    }
    fn ured(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::RED)
    }
    fn ugreen(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::GREEN)
    }
    fn uyellow(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::YELLOW)
    }
    fn ublue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::BLUE)
    }
    fn upurple(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::PURPLE)
    }
    fn ugrblue(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::GRBLUE)
    }
    fn ugray(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::GRAY)
    }
    fn uwhite(&self) -> BFRDYN {
        stylify(self.to_string().as_str(), Underline::WHITE)
    }
}

/// trait ColorBg256, Color256, ColorUl256 and ColorBold256 are
/// implemented for &str, &String and String
impl Color256 for &str {}
impl ColorBold256 for &str {}
impl ColorBg256 for &str {}
impl ColorUl256 for &str {}

/// trait ColorBg256, Color256, ColorUl256 and ColorBold256 are
/// implemented for &str, &String and String
impl Color256 for &String {}
impl ColorBold256 for &String {}
impl ColorBg256 for &String {}
impl ColorUl256 for &String {}

/// trait ColorBg256, Color256, ColorUl256 and ColorBold256 are
/// implemented for &str, &String and String
impl Color256 for String {}
impl ColorBold256 for String {}
impl ColorBg256 for String {}
impl ColorUl256 for String {}

// #### INFO: Dynamics


#[doc(hidden)]
/// helper function for internal use
fn stylify2<const S: usize>(text: &str, color: impl Formatter) -> BFRDYN<S> {
    let mut b: BFRDYN<S> = color.to_string().as_str().into();
    let _ = b.append_str(text.to_string().as_str());
    let _ = b.append_str(Special::DEF.to_string().as_str());
    b
}

///
/// This trait receive const generic to specify the buffer size
/// A trait that contain methods to format self with regular color
/// once this trait imported, &str, &String and String will
/// have extension method to format it with color on terminal.
/// # Example
/// ```
/// use consclr::{Color};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.dred::<512>());
/// ```
pub trait Color: ToString {
    fn dblack<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::BLACK)
    }
    fn dred<const S: usize>(&self) -> BFRDYN<S> { 
        stylify2(self.to_string().as_str(), Regular::RED)
    }
    fn dgreen<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::GREEN)
    }
    fn dyellow<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::YELLOW)
    }
    fn dblue<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::BLUE)
    }
    fn dpurple<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::PURPLE)
    }
    fn dgrblue<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::GRBLUE)
    }
    fn dgray<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::GRAY)
    }
    fn dwhite<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Regular::WHITE)
    }
}


impl Color for &str {}
impl Color for &String {}
impl Color for String {}


///
/// This trait receive const generic to specify the buffer size
/// A trait that contain methods to format self with color and bold
/// once this trait imported, &str, &String and String will
/// have extension method to format it with color+bold on terminal.
/// # Example
/// ```
/// use consclr::{ColorBold};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.dredb::<512>());
/// ```
pub trait ColorBold: ToString {
    fn dblackb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::BLACK)
    }
    fn dredb<const S: usize>(&self) -> BFRDYN<S> { 
        stylify2(self.to_string().as_str(), Bold::RED)
    }
    fn dgreenb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::GREEN)
    }
    fn dyellowb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::YELLOW)
    }
    fn dblueb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::BLUE)
    }
    fn dpurpleb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::PURPLE)
    }
    fn dgrblueb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::GRBLUE)
    }
    fn dgrayb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::GRAY)
    }
    fn dwhiteb<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Bold::WHITE)
    }
}


impl ColorBold for &str {}
impl ColorBold for &String {}
impl ColorBold for String {}


///
/// This trait receive const generic to specify the buffer size
/// A trait that contain methods to format self with colorful background
/// once this trait imported, &str, &String and String will
/// have extension method to format it with colorful background on terminal.
/// # Example
/// ```
/// use consclr::{ColorBg};
/// 
/// let text = "Lorem Ipsum";
/// println!("{}", text.dpurplebg::<512>());
/// ```
pub trait ColorBg: ToString {
    fn dblackbg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::BLACK)
    }
    fn dredbg<const S: usize>(&self) -> BFRDYN<S> { 
        stylify2(self.to_string().as_str(), Background::RED)
    }
    fn dgreenbg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::GREEN)
    }
    fn dyellowbg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::YELLOW)
    }
    fn dbluebg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::BLUE)
    }
    fn dpurplebg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::PURPLE)
    }
    fn dgrbluebg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::GRBLUE)
    }
    fn dgraybg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::GRAY)
    }
    fn dwhitebg<const S: usize>(&self) -> BFRDYN<S> {
        stylify2(self.to_string().as_str(), Background::WHITE)
    }
}


impl ColorBg for &str {}
impl ColorBg for &String {}
impl ColorBg for String {}
