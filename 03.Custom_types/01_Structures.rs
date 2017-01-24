// [3.1 Structures](http://rustbyexample.com/custom_types/structs.html)

/*
 There are three types of structures ("structs") that can be created using
 the struct keyword:

    Tuple structs, which are, basically, named tuples.
    The classic C structs
    Unit structs, which are field-less, are useful for generics.

*/

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

/*
Add a function rect_area which calculates the area of a rectangle
(try using nested destructuring).
*/

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle { p1: Point { x: ref x1, y: ref y1 },
                    p2: Point { x: ref x2, y: ref y2 }
                  } = *r;

    (x2 - x1) * (y2 - y1)
}

/*
Add a function square which takes a Point and a f32 as arguments,
and returns a Rectangle with its lower left corner on the point,
and a width and height corresponding to the f32.
*/
fn square(p: Point, size:f32) -> Rectangle {
    // p1 lower left corner on the point
    let (p1_x,p1_y)=(p.x, p.y);
    Rectangle { p1: Point{ x: p1_x, y: p1_y },
                p2: Point{ x: p1_x + size, y: p1_y + size }
              }
}
/* -- BROKEN:
fn square(pp: Point, s: f32) -> Rectangle {
    let p = &pp;
    Rectangle { p1: *p,
                p2: Point{ x: p.x + s,
                           y: p.y+s
                }
    }
}
*/


fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

   // area of a rectangle
   println!("area {}", rect_area(&_rectangle));

   // Square
   let point: Point = Point { x: 0.3, y: 0.4 };

   let sq = square(point, 10.0);
   println!("square {}", rect_area( &sq ) );
}


