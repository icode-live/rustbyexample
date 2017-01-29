// [7.5 match](http://rustbyexample.com/flow_control/match.html)
/*
Rust provides pattern matching via the `match` keyword,
which can be used like a C `switch`.
*/

fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13...19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
/* if
      // true => 1,

rustc 1.14.0 (e8a012324 2016-12-16)
error[E0004]: non-exhaustive patterns: `true` not covered
--> <anon>:19:24
|
19 | let binary = match boolean {
error: aborting due to previous error
*/

