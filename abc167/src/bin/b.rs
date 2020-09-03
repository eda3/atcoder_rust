use itertools::Itertools;
fn main() {
  proconio::input!{a:u64,b:u64,c:u64,k:u64};
  println!("{}",a.min(k)-0.max(k-a-b))
}