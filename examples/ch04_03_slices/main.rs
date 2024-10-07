fn main() {
    let input = "hello world asdf".to_string();
    println!("{}", first_word(&input));

    println!("{}", first_word_better("hello world asdf"));
}

fn first_word(x: &String) -> &str {
    let bytes = x.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &x[..i]; // end index is index of the space exclusive
        }
    }

    &x[..] // the whole string as a slice
}

// same as the above, but takes a str slice as input
// this is more general, because it works on String references, String slices, str literals and str slices
fn first_word_better(x: &str) -> &str {
    let bytes = x.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &x[..i];
        }
    }

    &x[..]
}

// the below does not compile because we can't have a immutable borrow used after a mutable borrow. Nice.

// pub fn borrow_example() {
//     let mut x = "asdasd asd asdasdasd asd".to_string();
//     let word = first_word(&x);

//     x.clear();
//     println!("{}", word);
// }
