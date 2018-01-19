

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use glass::Glass;

mod figures;
mod glass;


fn main() {
    println!("Hello, world!");

    let glass = Glass::new(15, 30);
}
