fn main() {
    let x: u8 = 255;
    // this does not compile, compiler detects int overflow
    // alternatively, if x were not a compile-time literal, this would panic under debug build, and wrap around under release build
    // let y = x + 10;

    // this wraps
    let y = x.wrapping_add(10);
    println!("{y}"); // 9

    // this returns None in case of overflow
    let y = x.checked_add(10);
    // {:?} is for types that have `Debug` but not `Display`
    println!("{:?}", y); // None

    // self-explanatory
    let (y, has_overflown) = x.overflowing_add(10);
    println!("({}, {})", y, has_overflown); // (9, true)

    let y = x.saturating_add(10);
    println!("{y}"); // 255

    let _ = 12;
}
