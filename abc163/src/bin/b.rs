fn main() {
  proconio::input!{n:i64,m:i64,a:[i64;m]};
  let x=n-a.iter().sum::<i64>();
  println!("{}",if x < 0{-1}else{x})
}
