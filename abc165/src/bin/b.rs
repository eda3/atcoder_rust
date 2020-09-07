fn main() {
  proconio::input!{mut x:usize};
  let mut i = 0;
  let mut c = 100;
  while c < x{
    i += 1;
    c = c + c/100;
  }
  println!("{}", i)
}