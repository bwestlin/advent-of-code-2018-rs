extern crate time;


use std::error::Error;
use time::*;

pub fn measure_exec<F>(f: F) -> Result<(), Box<Error>>
where F: FnOnce() -> Result<(), Box<Error>> {
    let start = precise_time_ns();

    f()?;

    let dur_ns = precise_time_ns() - start;
    println!("Exec time: {}ms", dur_ns / 1_000_000);

    Ok(())
}
