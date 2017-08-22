#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate neon;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use options::Options;

mod commands;
mod options;
mod heroku_api;

fn main() {
    let args = env::args();
    let options = Options::new(args);
    let mut stderr = std::io::stderr();
    println!("{:?}", options);

    if options.cmd_init {
        let cmd = commands::Init::new(options);
        cmd.execute().unwrap_or_else(|err| {
            writeln!(
                &mut stderr,
                "I/O Error: {}",
                err.description()
            ).expect("Could not write to stderr");
        });
    } else if options.cmd_register {
        let cmd = commands::Register::new(options);
        cmd.execute();
    } else if options.cmd_search {
        let cmd = commands::Search::new(options);
        cmd.execute();
    }
}