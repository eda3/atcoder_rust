fn main() {
  proconio::input!{x:i32,n:usize,p:[i32;n]};
  let mut ans = -200;
  for i in 0..101{
    if p.contains(&i){
      continue
    }
    if (i-x).abs()<(ans-x).abs(){
      ans = i
    }
  }
  println!("{}",ans)
}
