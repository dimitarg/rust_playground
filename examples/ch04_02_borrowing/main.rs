fn main() {}

pub fn multiple_mutable_refs_example() {
    let mut x = "asdf".to_string();
    let _a = &mut x;
    let _b = &mut x;

    // this will not compile because println! uses _a via a mutable reference, but it was borrowed again as a mutable reference via _b in the meantime
    // println!("{} {}", _a, _b);
}

pub fn multiple_mutable_refs_example_2() {
    let mut x = "asdf".to_string();
    let _a = &mut x;

    println!("{}", _a);

    // this compiles because _a no longer valid
    let _b: &mut String = &mut x;

    println!("{}", _b);
}

// the below does not compile, because we're attempting to return a reference to a value that goes out of scope and is invalid
// fn dangle() -> &String {
//     let x = "asdf".to_string();
//     &x
// }
