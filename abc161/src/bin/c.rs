fn main(){
  proconio::input!{n:u64,k:u64};
  let m = n % k;
  println!("{}",m.min(k-m))
}
