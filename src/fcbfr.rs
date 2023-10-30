//! Color features that enable BFRDYN to be printed with color
//! # How to activate the color feature
//! # Example
//! ```
//! use consclr::Color256;
//! use cbfr::BFRDYN;
//!
//! let b: BFRDYN = "some text".into();
//! 
//! println!("{}", b.red());
//! println!("{}", b.green());
//! println!("{}", b.blue());
//! ```

impl crate::Color256 for cbfr::BFRDYN {}
impl crate::ColorBold256 for cbfr::BFRDYN {}
impl crate::ColorBg256 for cbfr::BFRDYN {}
impl crate::ColorUl256 for cbfr::BFRDYN {}
impl crate::Color for cbfr::BFRDYN {}
impl crate::ColorBold for cbfr::BFRDYN {}
impl crate::ColorBg for cbfr::BFRDYN {}
impl crate::ColorUl for cbfr::BFRDYN {}

