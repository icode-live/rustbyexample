// [18.5.2 Wait](http://rustbyexample.com/std_misc/process/wait.html)

/*
If you'd like to wait for a process::Child to finish,
you must call Child::wait, which will return a process::ExitStatus.
*/

// wait.rs
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}


/*
$ rustc 010__Wait.rs && ./010__Wait
reached end of main
# `wait` keeps running for 5 seconds
# `sleep 5` command ends, and then our `wait` program finishes
*/

