use proconio::marker::*;
fn main(){
  proconio::input!{s:Chars,t:Chars};
  println!("{}",s.iter().zip(t).filter(|&(&s,t)|s!=t).count());
}