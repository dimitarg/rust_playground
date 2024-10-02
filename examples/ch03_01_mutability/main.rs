fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 123;
    println!("The value of y is {y}");
    // will not compile since let means immutable val
    // y = 222;

    // compiler requries explicit type annotation for `const`
    // cannot be mutated via `mut`
    // naming convention as in C / Java
    const SECONDS_IN_DAY: u32 = 60 * 60 * 24;
    println!("There are {} seconds in a day", SECONDS_IN_DAY)
}
