fn main() {
  proconio::input!{k:u64,a:u64,b:u64};
  for i in a..=b{
    if i%k==0{
      println!("OK");
      return
    }
  }
  println!("NG")
}