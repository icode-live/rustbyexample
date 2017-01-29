// [5.2 Inference](http://rustbyexample.com/cast/inference.html)

/*
The type inference engine is pretty smart.
It does more than looking at the type of the r-value during an initialization.

It also looks at how the variable is used afterwards to infer its type.
Here's an advanced example of type inference:
*/

fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}

/* with vec.push(elem); commented we get the error
 *
rustc 1.14.0 (e8a012324 2016-12-16)
error[E0282]: unable to infer enough type information about `_`
--> <anon>:6:19
|
6 | let mut vec = Vec::new();
|
= note: type annotations or generic parameter binding required
error: aborting due to previous error
timeout triggered!

with vec.push(elem);
No type annotation of variables is needed,
the compiler is happy and so is the programmer!
*/
