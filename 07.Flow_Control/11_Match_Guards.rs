// [7.5.2 Guards](http://rustbyexample.com/flow_control/match/guard.html)
/*
A match guard can be added to filter the arm.

The `if condition`

*/

fn main() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        // The ^ `if condition` part is a guard
        (x, y)  if x == y      => println!("These are twins"),
        (x, y)  if x + y == 0  => println!("Antimatter, kaboom!"),
        (x, _)  if x % 2 == 1  => println!("The first one is odd"),
        _                      => println!("No correlation..."),
    }
}

