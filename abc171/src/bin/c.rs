fn main() {
  proconio::input!{mut n:u64};
  let mut s=String::new();
  while n>0{
    n -= 1;
    s.insert(0,((n%26)as u8+b'a') as char);
    n /=26;
  }
  println!("{}",s);
}