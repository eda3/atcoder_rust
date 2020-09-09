fn main() {
  proconio::input!{mut n:usize};
  println!("{}",(0..=n).filter(|x|x%5!=0&&x%3!=0).sum::<usize>())
}