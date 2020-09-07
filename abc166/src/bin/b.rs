fn main(){
  proconio::input!{n:usize,k:usize};
  let mut has = vec![true;n];
  for _ in 0..k{
    proconio::input!{d:usize,a:[usize;d]};
    println!("d={:?}:a={:?}", d, a);
    for j in 0..d{
      println!("j={:?}", j);
      println!("a[j]-0={:?}", a[j]-1);
      has[a[j]-1] = false;
    }
    println!()
  }
  println!("{:?}", has.iter());
  println!("{:?}", has.iter().filter(|x|**x).count())
}