fn main(){
  proconio::input!{n:u64,r:u64};
  println!("{}",if n>=10{r}else{r+1000-100*n})
}