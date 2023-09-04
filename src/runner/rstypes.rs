use crate::runner::{rsprinter, rstrait};

enum Animal {
    Dog = 1,
    Cat = 2,
}

impl Default for Animal {
	fn default() -> Self {
		Animal::Cat
	}
}

#[allow(dead_code)]
struct Color (u8, u8, u8);

#[allow(dead_code)]
fn some_code_chunk() {
	println!("Hello world");
}

pub struct RsType;

impl rstrait::Runner for RsType {
	fn run(&self) {
		rsprinter::print_start();
		let is: bool = false;
		let _letter: char = 'a';
		let _word: &str = "word";
		let x: f32 = 21.1;
		let y: f32 = 21.1;
		let _z1: f64 = x as f64 + y as f64;
		let _that_vector: (f32, f32) = (x, y);
		let _a: Animal = Animal::Dog;
		if is {
			println!("it not");
			return;
		}

		let z: f32 = x + y;
		println!("thats what i thought");
		println!("z = {}", z);
	}
}
