fn main() {
  proconio::input!{mut n:u64,k:u64};
  let mut a = 0;
  while n > 0{
    n /= k;
    a += 1
  }
  println!("{}",a);
}