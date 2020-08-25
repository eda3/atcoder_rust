fn main(){
 proconio::input!{n:u64,a:[u64;n]}
 let mut m=0;
 let mut x=0;
 for &i in a.iter(){
  if i<m{x+=m-i;}
  m=m.max(i);
 }
 println!("{}",x);
}