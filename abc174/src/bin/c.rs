fn main(){
 proconio::input!{k:u64};
 let mut x=0;
 for i in 1..=k{
  x=(x*10+7)%k;
  if x ==0{
   println!("{}",i);
   return;
  }
 }
 println!("-1");
}
