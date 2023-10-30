# consclr
a rust library to make your console text colorful

# What's new (v0.2.2)
    -- enums on colors module now implement from AsRef<str> trait. 
    -- so now we can create color from str like this:
```Rust
fn main() {
    use consclr::prelude::Regular;
    let color: Regular = "yellow".into();
}
```
    -- new macro clr2!, this macro is similar to clr! unless clr2! will not add newline.

# What's new (v0.2.1)
    -- New traits ColorUl256 to colorized and add underline style to your text on terminal
    -- traits ColorUl for dynamic buffer with generic type still not available for current version but will be added in the future

# Quick start
```rust
use consclr::{prelude::*, ColorBg, Color, ColorBold};
use consclr::{clr, cprint, pchar};


fn main() {
    let a = "black"; let b = "red"; let c = "green";
    let d = "yellow"; let e = "blue"; let f = "purple";
    let g = "grblue"; let h = "gray"; let i = "white";

    println!("{}", a.black());
    println!("{}", b.red());
    println!("{}", c.green());
    println!("{}", d.yellow());
    println!("{}", e.blue());
    println!("{}", f.purple());
    println!("{}", g.grblue());
    println!("{}", h.gray());
    println!("{}", i.white());

    pchar!("~", 60, &Bold::GREEN.s());

    println!("{}", a.blackb());
    println!("{}", b.redb());
    println!("{}", c.greenb());
    println!("{}", d.yellowb());
    println!("{}", e.blueb());
    println!("{}", f.purpleb());
    println!("{}", g.grblueb());
    println!("{}", h.grayb());
    println!("{}", i.whiteb());

    pchar!("~", 60, &Bold::GREEN.s());

    println!("{}", a.bgblack());
    println!("{}", b.bgred());
    println!("{}", c.bggreen());
    println!("{}", d.bgyellow());
    println!("{}", e.bgblue());
    println!("{}", f.bgpurple());
    println!("{}", g.bggrblue());
    println!("{}", h.bggray());
    println!("{}", i.bgwhite());

    pchar!(".", 60, &Bold::GREEN.s());

    let f = Regular::GRBLUE.s();
    cprint!("
Programmers who are already working with low-level code can use Rust to raise their",
        &f);

    clr!("
Programmers who are already working with low-level code can use Rust to raise their
ambitions. For example, introducing parallelism in Rust is a relatively low-risk operation:
the compiler will catch the classical mistakes for you. And you can tackle more aggressive
optimizations in your code with the conﬁdence that you won’t accidentally introduce
crashes or vulnerabilities
        ", &Regular::RED.s());

    pchar!("!", 100, &Background::YELLOW.s());

    let long_text = "
Programmers who are already working with low-level code can use Rust to raise their
ambitions. For example, introducing parallelism in Rust is a relatively low-risk operation:
the compiler will catch the classical mistakes for you. And you can tackle more aggressive
optimizations in your code with the conﬁdence that you won’t accidentally introduce
crashes or vulnerabilities";

    //// with const generic you can manually specify buffer size
    //// for example, when you need to store long text data

    // println!("{}", long_text.green()); // text is too large, and program will panic
    println!("{}", long_text.dgreen::<512>()); // use const generic dgreen() instead from Color trait
    println!("{}", long_text.dblueb::<512>()); // use const generic dblueb() instead from ColorBold trait
    println!("{}", long_text.dpurplebg::<512>()); // use const generic dpurplebg() instead from ColorBg trait
}
```
