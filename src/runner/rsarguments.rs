use std::env;
use crate::runner::rsprinter;
use crate::runner::rstrait;

pub struct RsArguments;

impl rstrait::Runner for RsArguments {
	fn run(&self) {
		rsprinter::print_start();
		let args: Vec<String> = env::args().collect();
		for (i, arg) in args.iter().enumerate() {
			if i == 0 {
				println!("{}", arg);
				continue;
			}
			rsprinter::print_mid();
			match arg.parse::<i64>() {
				Ok(x) => {
					println! ("x:? = {:?}", x);
					println!("x:b = {:b}", x);
					println!("x:x = {:x}", x);
				},
				Err(_x) => {
					println!("{}", arg);
				},
			};
		}
	}

}
