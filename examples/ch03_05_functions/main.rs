fn main() {
    println!("{}", a(22)); // 35
}

// function with argument, returning ()
pub fn sth(x: i32, y: bool) {
    let _ = x;
    let _ = y;
    // this happens implicitly I assume
    // ()
}

// function with return type
fn a(x: i32) -> i32 {
    // the below being the return expression does not compile, because adding semicolon to an expression turns it into a statement
    // x + 13;
    // this is an expression as opposed to a statement
    x + 13
}
