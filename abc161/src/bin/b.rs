fn main(){
  proconio::input!{n:usize,m:usize,a:[isize;n]};
  println!("{:?}", n); // 4 n種類の賞品
  println!("{:?}", m); // 1 m個を選ぶ
  println!("{:?}", a); // [5, 4, 2, 1] 各賞品の投票数

  let total = a.iter().sum();
  println!("{}", total); // 12 合計投票数

  let count = a.iter().filter(|x|4 * (m as isize) * **x >= total).count();
  println!("{}", count); // 2

  println!("{}",if count>=m{"Yes"}else{"No"}) // Yes
}