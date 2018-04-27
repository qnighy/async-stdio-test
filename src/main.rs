extern crate tokio;
extern crate tokio_file_unix;

use std::sync::{atomic, Arc};
use std::time::{Duration, Instant};

use tokio::io;
use tokio::prelude::*;
use tokio::timer;
use tokio_file_unix::raw_stdin;

fn main() {
    tokio::run(future::lazy(main2));
}

fn main2() -> Box<Future<Item = (), Error = ()> + Send> {
    let i = Arc::new(atomic::AtomicIsize::new(0));
    let i2 = i.clone();

    let interval = timer::Interval::new(Instant::now(), Duration::from_secs(1));
    let task1 = interval
        .for_each(move |_| {
            println!("{}", i.load(atomic::Ordering::SeqCst));
            Ok(())
        })
        .map_err(|e| panic!("Interval errored: {}", e));

    tokio::executor::spawn(task1);

    let stdin = tokio_file_unix::File::new_nb(raw_stdin().unwrap()).unwrap();
    let stdin = stdin
        .into_reader(&tokio::reactor::Handle::current())
        .unwrap();
    let task2 = io::lines(stdin)
        .for_each(move |line| {
            i2.store(line.parse().unwrap(), atomic::Ordering::SeqCst);
            Ok(())
        })
        .map_err(|e| panic!("Lines errored: {}", e));

    Box::new(task2)
}
