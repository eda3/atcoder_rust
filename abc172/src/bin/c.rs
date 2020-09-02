fn main() {
  proconio::input!{n:usize,mut m:usize,k:usize,mut a:[usize;n],mut b:[usize;m]};
  a.insert(0,0);
  let mut t=b.iter().sum::<usize>();
  let mut x=0;
  for i in 0..a.len(){
    t+=a[i];
    while t>k&&m>0{
      m-=1;
      t-=b[m]
    }
    if t>k{break}
    x=x.max(m+i)
  }
  println!("{}",x)
}