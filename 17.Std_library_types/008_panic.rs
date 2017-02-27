// [17.6 panic!](http://rustbyexample.com/std/panic.html)
/*
The panic! macro can be used to generate a panic and start unwinding its stack.
While unwinding, the runtime will take care of freeing all the resources owned
by the thread by calling the destructor of all its objects.

Since we are dealing with programs with only one thread, panic! will cause
the program to report the panic message and exit.
*/

// Re-implementation of integer division (/)
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// The `main` task
fn main() {
    // Heap allocated integer
    let _x = Box::new(0i32);

    // This operation will trigger a task failure
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` should get destroyed at this point
}

/*
Let's check that panic! doesn't leak memory.

$ rustc 008_panic.rs && valgrind ./008_panic
==13401== Memcheck, a memory error detector
==13401== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==13401== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==13401== Command: ./008_panic
==13401==
thread 'main' panicked at 'division by zero', 008_panic.rs:15
note: Run with `RUST_BACKTRACE=1` for a backtrace.
==13401==
==13401== HEAP SUMMARY:
==13401==     in use at exit: 0 bytes in 0 blocks
==13401==   total heap usage: 0 allocs, 0 frees, 0 bytes allocated
==13401==
==13401== All heap blocks were freed -- no leaks are possible
==13401==
==13401== For counts of detected and suppressed errors, rerun with: -v
==13401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
*/


