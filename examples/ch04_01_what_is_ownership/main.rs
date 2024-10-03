fn main() {
    blah();
    copy_demo();
}

pub fn blah() {
    // the below does not compile, because we cannot allocate `str` on the stack, sinze it's not `Sized`
    // the reason is, even though it's a literal known at compile time, unlike arrays, there is no type for "str of size N",
    // i.e. "asdf" and "asdfasdf" have the same type.
    // let _: str = "asd";

    // this (the reference) is allocated on the stack
    let _: &str = "asd";

    // this is allocated on the heap, and can be mutated if declared mutable
    let mut x = String::from("asdf");
    x.push_str("asdf");

    println!("{x}"); // asdfasdf

    // here x goes out of scope, is not borrowed from outside and will be deallocated
}

pub fn asd() {
    let x = String::from("asdf");
    let _y = x;
    // below does not compile because x is moved to y (not copied / cloned) and so x becomes invalid
    // println!("{x}")
}

fn copy_demo() {
    // this is equivalent to String::from
    let x = "asd".to_string();
    // this performs a (deep copy). X isn't moved or borrowed, both are valid at this point
    let y = x.clone();

    println!("x: {x}, y: {y}");
}

pub fn move_ok_demo() {
    let x = (1, true, "asd");
    let y = x;

    // x is ok to access because it was not moved - it's a tuple of scalars so it's stored on the stack and implements (shallow) `Copy`
    println!("{}", x.0);
    println!("{}", y.0);
}

pub fn functions_and_ownership_demo() {
    let x = "hello".to_string();
    takes_ownership(x);

    // this will not compile since x was moved at this point
    //println!("{x}");

    let x = 123;
    makes_copy(x);
    // ok since it's copied.
    println!("{x}");
}

fn takes_ownership(x: String) {
    let _ = x;
}

fn makes_copy(x: i32) {
    let _ = x;
}

pub fn we_need_references_demo() {
    let x = "asd".to_string();
    let len = calc_len_wrong(x);

    println!("{len}");
    // this will not compile because x was moved to `calc_len_wrong`
    // instead we should have taken s inside that function as reference
    // println!("{x}")
}

fn calc_len_wrong(s: String) -> usize {
    s.len()
}
