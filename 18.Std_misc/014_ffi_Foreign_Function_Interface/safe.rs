// [18.8 Foreign Function Interface](http://rustbyexample.com/std_misc/ffi.html)

/*
Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign
functions must be declared inside an extern block annotated with a #[link]
attribute containing the name of the foreign library.

Since calling foreign functions is considered unsafe, it's common
to write safe wrappers around them.

see [ffi.rs](./ffi.rs)
*/

// safe.rs
use std::fmt;

#[link(name = "m")]
extern {
    fn ccosf(z: Complex) -> Complex;
}

// safe wrapper
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = 0 + 1i
    let z = Complex { re: 0., im: 1. };

    println!("cos({:?}) = {:?}", z, cos(z));
}

// Minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

/*
$ rustc safe.rs && ./safe
cos(0+1i) = 1.5430806+0i
*/

