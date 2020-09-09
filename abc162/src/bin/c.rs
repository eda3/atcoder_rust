use num_integer::gcd;
use itertools::iproduct;
fn main() {
  proconio::input!{n:i32};
  println!("{}",iproduct!(1..=n,1..=n,1..=n).fold(0,|sum,x|sum+gcd(x.0,gcd(x.1,x.2))))
}