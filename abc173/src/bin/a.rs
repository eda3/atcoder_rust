fn main(){
 proconio::input!{n:u16};
 let x=n%1000;
 match x{
  0=>{println!("{}",x)}
  _=>{println!("{}",1000-x)}
 }
}
