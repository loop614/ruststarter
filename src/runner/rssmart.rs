use std::rc::Rc;
use std::sync::Arc;
use crate::runner::rsprinter;
use crate::runner::rstrait::Runner;

pub struct RsSmart;

impl Runner for RsSmart {
    fn run(&self) {
        rsprinter::print_start();
        let x: u64 = 1;
        let a: Rc<&u64> = Rc::new(&x);
        let f = Rc::clone(&a);
        println!("rc strong count for a {}", Rc::strong_count(&a));
        std::mem::drop(a);
        println!("rc strong count for a {}", Rc::strong_count(&f));
        std::mem::drop(f);
        rsprinter::print_lines();

        let y: u64 = 2;
        let b: Arc<&u64> = Arc::new(&y);
        let e = Arc::clone(&b);
        println!("rc strong count for a {}", Arc::strong_count(&b));
        std::mem::drop(b);
        println!("rc strong count for a {}", Arc::strong_count(&e));
        std::mem::drop(e);
        rsprinter::print_lines();
    }
}
