#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate dirs;
extern crate chrono;
extern crate clap;
extern crate colored;

use clap::{App, Arg};
use colored::*;

mod linggle;
mod storage;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query")
            .help("The string you are querying")
            .value_name("QUERY")
            .required(true))
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let result = linggle::query(query).unwrap();
    for (idx, ngram) in result.ngrams.iter().enumerate() {
        let index = format!("{:>2}.", idx + 1).as_str().yellow();
        let count =format!("{}", ngram.count).as_str().magenta();
        let percent = format!("{:.2}%", 100_f32 * ngram.count as f32 / result.total as f32).as_str().blue();
        println!("{} {}|{} \t{}", index, count, percent, ngram.text);
    }
}
