// [7.1 if/else](http://rustbyexample.com/flow_control/if_else.html)
/*
Branching with if-else is similar to other languages.
Unlike many of them, the boolean condition doesn't need to be surrounded
by parentheses, and each condition is followed by a block.

if-else conditionals are expressions,
and, all branches must return the same type.

*/

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

/* if
            n / 2;
            // TODO ^ Try suppressing this expression with a semicolon.

rustc 1.14.0 (e8a012324 2016-12-16)
error[E0308]: if and else have incompatible types
--> <anon>:13:9
|
13 | if n < 10 && n > -10 {
|
= note: expected type `{integer}`
= note: found type `()`
error: aborting due to previous error
*/

