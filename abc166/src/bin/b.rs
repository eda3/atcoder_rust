fn main(){
  proconio::input!{n:usize,k:usize};
  let mut has = vec![true;n];
  for _ in 0..k{
    proconio::input!{d:usize,a:[usize;d]};
    for j in 0..d{
      has[a[j]-1] = false;
    }
  }
  println!("{:?}", has.iter().filter(|x|**x).count())
}