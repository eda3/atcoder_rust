fn main() {
  proconio::input!{n:u64,k:usize,mut p:[u64;n]};
  p.sort();
  println!("{}",p[0..k].iter().sum::<u64>())
}