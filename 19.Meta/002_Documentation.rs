// [19.1 Documentation](http://rustbyexample.com/meta/doc.html)

/*
Doc comments are very useful for big projects that require documentation.
When running Rustdoc, these are the comments that get compiled
into documentation.
They are denoted by a ///, and support [Markdown](https://en.wikipedia.org/wiki/Markdown).
*/

#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}

/*
To run the tests, first build the code as a library, then tell rustdoc where
to find the library so it can link it into each doctest program:

rustc doc.rs --crate-type lib
rustdoc --test --extern doc="libdoc.rs"

(When you run cargo test on a library crate, Cargo will automatically
 generate and run the correct rustc and rustdoc commands.)
*/

