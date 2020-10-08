fn main(){
  proconio::input!{k:u64,n:usize,a:[u64;n]};
  println!("{:?}",k);
  println!("{:?}",n);
  println!("{:?}",a);

  // 湖を囲う家のうち、家と家の距離が一番長い地点を特定
  let mut c = a[0] + k - a[n-1];
  for i in 0..n-1{
    c = c.max(a[i+1] - a[i]);
    println!("{}",c);
  }
  println!("{:?}",k-c);
}
