fn main() {
  proconio::input!{x:u64,y:u64};
  println!("{}",if x*2<=y&&y<=x*4&&y%2==0{"Yes"}else{"No"});
}
