fn main() {
  proconio::input!{n:[u8;5]};
  println!("{}",n.iter().position(|x|*x==0).unwrap()+1);
}