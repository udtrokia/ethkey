#[macro_use]
extern crate serde_derive;

pub mod cli;
pub mod panic_hook;

pub mod hello {
    pub fn he() {
        println!("hello, world!");
    }
}

