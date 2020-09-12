fn main(){
  proconio::input!{n:usize,mut a:[u128;n]};
  let mut ans = 1;
  a.sort();
  for i in a{
    ans*=i;
    if(10 as u128).pow(18)<ans{
      println!("{}",-1);
      return
    }
  }
  println!("{}",ans)
}