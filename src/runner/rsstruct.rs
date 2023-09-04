use crate::runner::{rsprinter, rstrait};
use std::mem;
struct RsChar {
    _c: char
}

struct RsI8 {
    _i: i8
}

pub struct RsStruct {
    pub a: u64,
    pub b: i64,
}

impl rstrait::Runner for RsStruct {
	fn run(&self) {
        rsprinter::print_start();
        println!("char has {} bytes", mem::size_of::<char>());
        println!("Struct with one char has {} bytes", mem::size_of::<RsChar>());
        println!("i8 has {} bytes", mem::size_of::<i8>());
        println!("Struct with one i8 has {} bytes", mem::size_of::<RsI8>());
    }
}
