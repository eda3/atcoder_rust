fn main(){
  proconio::input!{a:i64,b:i64,_:i64,k:i64};
  println!("{}",a.min(k)-0.max(k-a-b));
}