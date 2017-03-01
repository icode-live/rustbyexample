// [18.4.1 open](http://rustbyexample.com/std_misc/file/open.html)

/*
The open static method can be used to open a file in read-only mode.

A File owns a resource, the file descriptor and takes care of closing
the file when it is droped.
*/
// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}



/*
Here's the expected successful output:

$ echo "Hello World!" > hello.txt
$ rustc 006__open.rs && ./006__open
hello.txt contains:
Hello World!

(You are encouraged to test the previous example under different failure
 conditions: hello.txt doesn't exist, or hello.txt is not readable, etc.)
*/

