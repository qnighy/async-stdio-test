#![feature(proc_macro, generators)]

extern crate futures_await as futures;
extern crate tokio;
extern crate tokio_file_unix;

use std::sync::{atomic, Arc};
use std::time::{Duration, Instant};

use futures::prelude::*;
use tokio::io;
use tokio::prelude::*;
use tokio::timer;
use tokio_file_unix::raw_stdin;

fn main() {
    tokio::run(future::lazy(main2));
}

#[async]
fn main2() -> Result<(), ()> {
    let i = Arc::new(atomic::AtomicIsize::new(0));

    await!(tokio::executor::spawn(task1(i.clone())).into_future())?;

    await!(task2(i))
}

#[async]
fn task1(i: Arc<atomic::AtomicIsize>) -> Result<(), ()> {
    let interval = timer::Interval::new(Instant::now(), Duration::from_secs(1));
    #[async]
    for _ in interval.map_err(|e| panic!("Interval errored: {}", e)) {
        println!("{}", i.load(atomic::Ordering::SeqCst));
    }

    Ok(())
}

#[async]
fn task2(i: Arc<atomic::AtomicIsize>) -> Result<(), ()> {
    let stdin = tokio_file_unix::File::new_nb(raw_stdin().unwrap()).unwrap();
    let stdin = stdin
        .into_reader(&tokio::reactor::Handle::current())
        .unwrap();

    #[async]
    for line in io::lines(stdin).map_err(|e| panic!("Lines errored: {}", e)) {
        i.store(line.parse().unwrap(), atomic::Ordering::SeqCst);
    }

    Ok(())
}
