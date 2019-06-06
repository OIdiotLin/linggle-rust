#[macro_use]
extern crate lazy_static;
extern crate reqwest;

mod linggle;

fn main() {
    let res = linggle::query("what _ day").unwrap();
    println!("{:?}", res);

}
