fn main() {
  proconio::input!{n:u64};
  println!("{}", n / 500 * 1000 + ((n % 500) / 5) * 5)
}