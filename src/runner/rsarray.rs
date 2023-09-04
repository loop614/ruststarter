use crate::runner::{rsprinter, rstrait};

pub struct RsArray;

impl rstrait::Runner for RsArray {
	fn run(&self) {
		rsprinter::print_start();
		let mut array: [i32; 3] = [1, 2, 3];
		array[0] = 3;
		array[1] = 2;
		array[2] = 1;
		for number in array {
			println!("{:?}", number);
		}

		let _boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
		let mut this_vec: Vec <i32> = vec![1, 2];
		this_vec.push(4);

		for i in this_vec.iter() {
			println!("{:?}", i);
		}
		println!("{:?}", this_vec.get(3));
	}
}
