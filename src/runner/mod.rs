use crate::runner::rstrait::Runner;

mod rsarray;
mod rsarguments;
mod rstypes;
mod rstrait;
mod rsprinter;
mod rsstruct;
mod rsmacro;
mod rsownership;
mod rsfuture;
mod rsrequest;

pub fn run() {
    let runners: Vec<Box<dyn Runner>> = vec![
        Box::new(rsarguments::RsArguments),
        Box::new(rsarray::RsArray),
        Box::new(rstypes::RsType),
        Box::new(rsstruct::RsStruct{ a: 1, b: 2 }),
        Box::new(rsmacro::RsMacro),
        Box::new(rsownership::RsOwnership),
    ];

    for runner in runners.iter() {
        runner.run();
    }
}

pub async fn async_run() {
    _ = rsrequest::RsRequest {}.async_run().await.expect("TODO: panic message");
    rsfuture::RsFuture {}.async_run().await;
}
