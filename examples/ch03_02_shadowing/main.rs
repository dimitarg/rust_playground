fn main() {
  let x = 5;

  // shadowing, as opposed to mutability
  let x = x + 1;

  {
      //shadowing in an inner scope
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}"); // 12
  }

  // expectedly, the outer scope will not see the inner scope
  println!("The value of x is: {x}"); // 6


}