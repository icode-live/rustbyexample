// [7.3 while](http://rustbyexample.com/flow_control/while.html)
/*
The while keyword can be used to loop until a condition is met.

Let's write the infamous FizzBuzz using a while loop.

Count from 1 to 100. Any number divisible by three is replaced by the word fizz
and any divisible by five by the word buzz.
Numbers divisible by both become fizz buzz.
*/

fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

