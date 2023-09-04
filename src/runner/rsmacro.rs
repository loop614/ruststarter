use crate::runner::rsprinter;
use crate::runner::rstrait;

macro_rules! foo {
    () => { $l - 1 };
    ($l:expr) => { $l - 1 };
}

macro_rules! bar {
    ($l:expr) => {foo!($l + 1)};
}

pub struct RsMacro;

impl rstrait::Runner for RsMacro {
    fn run(&self) {
        rsprinter::print_start();
        println!("macro foo!(3) = {:?}", bar!(3));
    }
}
