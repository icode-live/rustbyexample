// [11.3.1 Custom](http://rustbyexample.com/attribute/cfg/custom.html)

/*
Some conditionals like target_os are implicitly provided by rustc,
but custom conditionals must be passed to rustc using the --cfg flag.

*/

// custom.rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}


fn main() {
    conditional_function();
}


/*
Without the custom cfg flag:

$ rustc custom.rs && ./custom
No such file or directory (os error 2)

With the custom cfg flag:

$ rustc --cfg some_condition custom.rs && ./custom
condition met!

*/
