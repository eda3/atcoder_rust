fn main() {
  proconio::input!{n:usize,a:[usize;n-1]};
  let mut x:Vec<usize> = vec![0;n];
  // println!("{:?}",n);
  // println!("{:?}",a);
  // println!("{:?}",x);
  for i in a.iter(){
    x[i-1] += 1;
  }
  x.iter().for_each(|x|println!("{}",x))
}