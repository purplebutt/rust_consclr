use consclr::pchar;
use consclr::cprint;
use consclr::ceprint;
use consclr::colorized;

use consclr::colors::Bold;
use consclr::colors::Regular;
use consclr::colors::Formatter;

fn main() {
    let f = Regular::GRBLUE.s();

    pchar!(".", 60, &Bold::GREEN.s());
    cprint!( "Message to stdout", &f);
    ceprint!( "Error message to stderr", &f);
}
