fn main() {
    let tuple = (1, 4.0, true);
    // `:.1` stands for float format with 1-digit precision after decimal point
    // `tuple.1` is an accessor for the second element of the tuple (0-based naming)
    println!("{:.1}", tuple.1); // 4.0

    let (x, _, _) = (1, 4.0, true); // tuple destructuring
    println!("{x}"); // 1

    let x = (); // the 0-tuple, Unit
    println!("{:?}", x); // ()

    // arrays are fixed-length
    // the type contains the length
    // is stack allocated
    // elements cannot be mutated since it was declared via `let`
    let _: [i32; 5] = [1, 2, 3, 4, 5];

    // this fills a 100-element array with 1-s
    let x = [1; 100];
    println!("{}", x[99]); // 1

    // println!("{}", x[100]); // does not compile, alternatively would panic if we accessed with a number that's not a literal
    // x[-1] // also does not compile, nice!

    // does not compile
    // x[1] = 20;
}
