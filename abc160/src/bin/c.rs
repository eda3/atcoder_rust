fn main() {
  proconio::input! {k:u64,n:usize,a:[u64;n]};

  let mut c = a[0] + k - a[n - 1];
  for i in 0..n - 1 {
    c = c.max(a[i + 1] - a[i]);
  }
  println!("{}", k-c)
}