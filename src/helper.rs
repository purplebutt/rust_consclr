use cbfr::prelude::BFRDYN;
use crate::Formatter;
use crate::colors::Special;


/// set opening format
/// # Example
/// ```
/// use consclr::helper::{op, cl};
/// use consclr::Color256;
/// use consclr::colors::Regular;
///
/// let f = Regular::GREEN;
/// let some_text = "A red panda stand on his rear foot";
/// println!("{}{}{}", op(f), some_text, cl());
/// ```
pub fn op(format: impl Formatter) -> BFRDYN {
    let b: BFRDYN = format.to_string().as_str().into();
    b
}

/// reset current terminal format to default
/// causing next output stream to show with default color
/// # Example
/// ```
/// use consclr::helper::{op, cl};
/// use consclr::Color256;
/// use consclr::colors::Regular;
///
/// let f = Regular::GREEN;
/// let some_text = "A red panda stand on his rear foot";
/// println!("{}{}{}", op(f), some_text, cl());
/// ```
pub fn cl() -> BFRDYN {
    let b: BFRDYN = Special::DEF.to_string().as_str().into();
    b
}

