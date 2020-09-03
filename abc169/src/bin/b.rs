fn main(){
  proconio::input!{n:u64,mut a:[usize;n]};
  let mut ans = 1;
  a.sort();
  for i in a{
    ans*=i;
    if(10 as usize).pow(18)<ans{
      println!("{}",-1);
      return
    }
  }
  println!("{}",ans)
}
