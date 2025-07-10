
use std::fmt::Display;

pub use strf_macros::f;

pub fn eprint(args: impl Display) {
    eprint!("{args}")
}

pub fn eprintln(args: impl Display) {
    eprintln!("{args}")
}

pub fn print(args: impl Display) {
    print!("{args}")
}

pub fn println(args: impl Display) {
    println!("{args}")
}

#[inline(always)]
pub fn panic(args: impl Display) {
    panic!("{args}")
}

#[inline(always)]
pub fn todo(args: impl Display) {
    todo!("{args}")
}

#[inline(always)]
pub fn unimplemented(args: impl Display) {
    unimplemented!("{args}")
}

#[inline(always)]
pub fn unreachable(args: impl Display) {
    unreachable!("{args}")
}
