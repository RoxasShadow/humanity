extern crate humanity;
use humanity::{Humanity, GitHub};

use std::env;

fn main() {
    if let Some(repository) = env::args().nth(1) {
        let humanity = Humanity::get_from::<GitHub>(&*repository);
        println!("{}", humanity.to_string());
    }
    else {
        panic!("Usage: humanity <owner/repository>");
    }
}
