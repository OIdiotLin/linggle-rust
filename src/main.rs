#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate dirs;
extern crate chrono;
extern crate clap;

use clap::{App, Arg};

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
        println!("{index:>2}. ({count}|{percent:.2}%) {text}",
                 index = idx + 1,
                 text = ngram.text,
                 count = ngram.count,
                 percent = 100_f32 * ngram.count as f32 / result.total as f32);
    }
}
