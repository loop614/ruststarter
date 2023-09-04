use crate::runner::rsprinter;
use crate::runner::rstrait;

fn take_my_vec(my_vec: &mut Vec<i32>) {
    my_vec.push(5);
}

pub struct RsOwnership;
impl rstrait::Runner for RsOwnership {
    fn run(&self) {
        rsprinter::print_start();
        let mut our_vec: Vec<i32> = vec![1, 2, 3];
        take_my_vec(&mut our_vec);
        our_vec.push(1);

        for one in our_vec.iter() {
            println!("{}", one);
        }
    }
}
