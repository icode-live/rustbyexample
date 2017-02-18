// [13.1 RAII](http://rustbyexample.com/scope/raii.html)
/*
Variables in Rust do more than just hold data in the stack:
they also own resources,
     e.g. Box<T> owns memory in the heap.

Rust enforces RAII (Resource Acquisition Is Initialization), so whenever
an object goes out of scope, its destructor is called
and its owned resources are freed.

This behavior shields against resource leak bugs,
so you'll never have to manually free memory
or worry about memory leaks again!

Here's a quick showcase:
*/

// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}

/*
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== For counts of detected and suppressed errors, rerun with: -v
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)

No leaks here!

$ rustc 002_RAII.rs && valgrind ./002_RAII
==10027== Memcheck, a memory error detector
==10027== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.
==10027== Using Valgrind-3.11.0 and LibVEX; rerun with -h for copyright info
==10027== Command: ./002_RAII
==10027==
==10027==
==10027== HEAP SUMMARY:
==10027==     in use at exit: 0 bytes in 0 blocks
==10027==   total heap usage: 0 allocs, 0 frees, 0 bytes allocated
==10027==
==10027== All heap blocks were freed -- no leaks are possible
==10027==
==10027== For counts of detected and suppressed errors, rerun with: -v
==10027== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

*/

