fn main() {
  proconio::input!{mut a:u64,b:u64,mut c:u64,d:u64};
  let mut f = true;
  while 0 < a && 0 < c{
    if f{
      c -= b;
      f = false
    }else{
      a -= d;
      f = true
    }
  }
  println!("{}", if a==0{"No"}else{"Yes"})
}
