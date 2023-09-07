use crate::runner::rsprinter;
use crate::runner::rstrait::Runner;

pub struct RsResult;
impl Runner for RsResult {
    fn run(&self) {
        rsprinter::print_start();
        if let Ok(answer) = this_just_wont_work(1) {
            println!("{}", answer);
        } else {
            println!("i guess it wasnt ok");
        }

        match this_just_wont_work(10) {
            Ok(answer) => {
                println!("{}", answer);
            },
            Err(e) => {
                println!("{}", e);
            },
        };

        let b: Result<String, String> = this_just_wont_work(5);
        if b == Ok("well it did".to_string()) {
            println!("cant believe this is the only way to compare Result using if");
        }

        if b == Err("it is what it is".to_string()) {
            println!("cant believe this is the only way to compare Result using if");
        }
    }
}

fn this_just_wont_work(i: u64) -> Result<String, String> {
    if i < 5 {
        return Err("it is what it is".to_string());
    }

    return Ok("well it did".to_string());
}
