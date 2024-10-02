fn main() {
    println!(
        "{} celsius is {} farenheit.",
        45.0,
        celsius_to_farenheit(45.0)
    );
    println!(
        "{} farenheit is {} celsius.",
        113.0,
        farenheit_to_celsius(113.0)
    );

    println!("the 0th fibonacci number is {}", fibonacci(0));
    println!("the 1st fibonacci number is {}", fibonacci(1));
    println!("the 6th fibonacci number is {}", fibonacci(6));
    println!("the 100th fibonacci number is {}", fibonacci(100));
}

// °C = (°F - 32) × 5/9
// °F = (9/5) °C+32

fn celsius_to_farenheit(temp_celsius: f64) -> f64 {
    ((9.0 / 5.0) * temp_celsius) + 32.0
}

fn farenheit_to_celsius(temp_farenheit: f64) -> f64 {
    (temp_farenheit - 32.0) * (5.0 / 9.0)
}

// not sure what the book authors expect
// vec wasn't mentioned thus far
// recursion wasn't, either
// maybe Binet's Formula, but that uses exponentiation which wasn't mentioned either
// the below code uses vec, and has the problem it does not handle numeric overflow
fn fibonacci(n: usize) -> i128 {
    let mut xs = vec![0; n + 1];

    for i in 0..=n {
        let fib_i = if i == 0 {
            0
        } else if i == 1 {
            1
        } else {
            xs[i - 2] + xs[i - 1]
        };
        xs[i] = fib_i
    }

    xs[n]
}
