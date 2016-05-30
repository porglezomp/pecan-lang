#[macro_use]
extern crate nom;
extern crate core;

pub mod parser;
use parser::*;

fn main() {
    let ast = program(b"fn fib(n: I32) -> I32 {
  let a: I32 = 0;
  let b: I32 = 1;
  for i: I32 in 0..n {
    let c: I32 = b;
    b = (a + b);
    a = c;
  }
  return a;
}");
    println!("{:?}", ast);
}
