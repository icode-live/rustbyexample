// [7.4 for and range](http://rustbyexample.com/flow_control/for.html)
/*
The for in construct can be used to iterate through an Iterator.
One of the easiest ways to create an iterator
is to use the range notation a..b.

This yields values from a (inclusive) to b (exclusive) in steps of one.

Let's write FizzBuzz using `for`.

Count from 1 to 100. Any number divisible by three is replaced by the word fizz
and any divisible by five by the word buzz.
Numbers divisible by both become fizz buzz.
*/
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
