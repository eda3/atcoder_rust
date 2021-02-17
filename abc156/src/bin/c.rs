fn main() {
  proconio::input!{m:i32,a:[i32;m]};
  let &min = a.iter().min().unwrap();
  let &max = a.iter().max().unwrap();
  let mut min_power = std::i32::MAX;
  for p in min..max {
    let power =
      a.iter()
        .map(|n|(n-p)*(n-p))
        .fold(0,|sum,val|sum+val);
    if power < min_power {
      min_power = power;
    }
  }
  println!("{}",min_power);
}