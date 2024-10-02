fn main() {
    println!("{}", is_even(1));
    println!("{}", is_even(2));
    print_if_even(1);
    print_if_even(2);

    println!("{}", stupid_add_ten(30)); // 40
    println!("{}", stupid_add_ten_variant(40)); // 50

    println!("{}", break_label()); // 1001

    while_example();

    let xs = [1, 2, 3, 4, 5];
    println!("{}", for_in_example(&xs)); // 6
}

fn is_even(x: i32) -> bool {
    // if expression
    if x % 2 == 0 {
        true
    } else {
        false
    }
}

fn print_if_even(x: i32) {
    // this is an if-statement, ok to miss the else branch
    if x % 2 == 0 {
        println!("{} is even", x)
    }
}

pub fn demo(blah: bool) -> i32 {
    // because if is an expression, it can be the right-hand side of an assignment
    let x = if blah { 17 } else { 42 };
    x + 13
}

pub fn better_than_scala(cond: bool) {
    let _ = cond;

    // the below does not compile, the compiler does not attempt to unify a type between i32 and string
    // let _ = if cond { 3 } else { "four" };
}

fn stupid_add_ten(x: i32) -> i32 {
    let mut acc = 0;

    let to_add = loop {
        acc = acc + 1;
        if acc == 10 {
            break acc;
        }; // breaks the loop and returns acc
    };
    x + to_add
}

fn stupid_add_ten_variant(x: i32) -> i32 {
    let mut acc = 0;

    loop {
        acc = acc + 1;
        if acc == 10 {
            return acc + x;
        }; // breaks the whole function and returns acc
    } // note this is a statement but still compiles
}

fn break_label() -> i32 {
    let mut acc = 1;
    'outer: loop {
        acc = acc * 2;
        loop {
            acc = acc + 1;
            if acc > 1000 {
                break 'outer acc;
            }
        }
    }
}

fn while_example() {
    let mut x = 3;
    while x > 0 {
        println!("{x}");
        x = x - 1;
    }
    println!("liftoff!")
}

fn for_in_example(xs: &[i32]) -> i32 {
    // this means array reference
    let mut acc = 0;
    for x in xs {
        if x % 2 == 0 {
            acc = acc + x;
        };
    }
    acc
}
