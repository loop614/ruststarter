use crate::runner::rsprinter;
use crate::runner::rstrait;

#[derive(Clone, Copy)]
struct SomeData {
    a: u64,
}

fn mutate_struct(mut data: SomeData) {
    data.a = 2;
}

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

        let something: SomeData = SomeData{ a: 1 };
        mutate_struct(something);
        mutate_struct(something.clone());
        mutate_struct(something);
        mutate_struct(something.clone());
    }
}
