use consclr::{prelude::*, ColorBg, Color, ColorBold, to_color};
use consclr::{clr, clr2, cprint, pchar};

fn main() {
    let a = "black"; let b = "red"; let c = "green";
    let d = "yellow"; let e = "blue"; let f = "purple";
    let g = "grblue"; let h = "gray"; let i = "white";

    
    clr!("This is green text", "green");
    let color: Bold = "yellow".into();
    clr!("clr => Yellow text with bg grayblue", &color.s(), &Background::GRBLUE.s());
    clr2!("clr2 => Yellow text with bg grayblue", "yellow");

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

    // println!("{}", long_text.green()); // text is too large
    println!("{}", long_text.dgreen::<512>()); // use dynamic dgreen() instead from color trait
    println!("{}", long_text.dblueb::<512>()); // use dynamic dgreen() instead from color trait
    println!("{}", long_text.dpurplebg::<512>()); // use dynamic dgreen() instead from color trait
}

