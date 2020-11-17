fn main(){
  proconio::input!{a:i32,b:i32};
  println!("{}",
    (0..1010)
      .filter(|x| x * 8 / 100 == a && x * 10 / 100 == b)
      .nth(0)
      .unwrap_or(-1)
  )
}