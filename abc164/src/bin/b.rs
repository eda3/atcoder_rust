fn main() {
  proconio::input!{mut a:u64,b:u64,mut c:u64,d:u64};
  let x = (c+b-1)/b;
  let y = (a+d-1)/d;
  println!("{}", if x<=y{"Yes"}else{"No"})
}