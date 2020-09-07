fn main() {
  proconio::input!{a:u64,mut s:[String;a]};
  s.sort();
  s.dedup();
  println!("{}", s.iter().count())
}