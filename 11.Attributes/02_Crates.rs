// [11.2 Crates](http://rustbyexample.com/attribute/crate.html)

/*
The crate_type attribute can be used to tell the compiler whether a crate
is a binary or a library (and even which type of library),
and the crate_name attribute can be used to set the name of the crate.
*/

// lib.rs
// This crate is a library
#![crate_type = "lib"]

// The library is named "rary"
#![crate_name = "rary"]


pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

/*
When the crate_type attribute is used, we no longer need to pass
the --crate-type flag to rustc.

$ rustc lib.rs
$ ls lib*
library.rlib

*/
