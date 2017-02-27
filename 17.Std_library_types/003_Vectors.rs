// [17.2 Vectors](http://rustbyexample.com/std/vec.html)
/*
Vectors are re-sizable arrays.
Like slices, their size is not known at compile time,
but they can grow or shrink at any time.

A vector is represented using 3 words: a pointer to the data, its length,
and its capacity.
The capacity indicates how much memory is reserved for the vector.
The vector can grow as long as the length is smaller than the capacity.
When this threshold needs to be surpassed, the vector is reallocated with a
larger capacity.
*/

fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    collected_iterator.push(0);
    // FIXME ^ Comment out this line

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    println!("Fourth element: {}", xs[3]);
}

/*
More Vec methods can be found under
the [std::vec module](http://doc.rust-lang.org/std/vec/)
*/

