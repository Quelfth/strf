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

pub trait Print: Display {
    fn print(&self) { print!("{self}") }
    fn println(&self) { println!("{self}") }
    fn eprint(&self) { eprint!("{self}") }
    fn eprintln(&self) { eprintln!("{self}") }
}
impl<T: Display> Print for T {}

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
