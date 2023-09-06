use std::{future, io};
use std::thread;
use crate::runner::rsprinter;

pub struct RsFuture;
impl RsFuture {
    pub async fn async_run(&self) {
        rsprinter::print_start();
        println!("Hello world");
        let _x = foo().await;
        let _z = foo();
        let _a = foo2().await;
        let _b = foo2();

        let read_from_terminal = thread::spawn(move || {
            let mut buffer = String::new();
            println!("do you know who i am: ");
            io::stdin().read_line(&mut buffer).expect("input error");

            return buffer.clone();
        });

        let terminal_result = read_from_terminal.join();
        match terminal_result {
            Ok(x) => {
                match x.trim_end().trim_start() {
                    "614" => println!("correct"),
                    "loop" => println!("correct"),
                    "loop614" => println!("correct"),
                    _ => println!("not correct")
                };
            },
            Err(e) => panic!("{:?}", e)
        }
    }
}

async fn foo() -> usize {
    println!("foo called");
    0
}

fn foo2() -> impl future::Future<Output = usize> {
    async {
        println!("foo called");
        0
    }
}
