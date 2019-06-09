#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate dirs;
extern crate chrono;

mod linggle;
mod storage;

fn main() {
    let res = linggle::query("what _ day").unwrap();
    println!("{:?}", res);

}
