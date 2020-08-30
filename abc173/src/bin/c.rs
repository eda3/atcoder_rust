use proconio::{input, marker::Bytes};
fn main() {
  input!(h:usize, w:usize, k:usize, c:[Bytes; h]);
  println!("h={:?}", h);
  println!("1<<h={:?}", 1<<h);
  println!("{:?}", w);
  println!("{:?}", k);
  println!("{:?}", c);

  let mut r = 0;
  for a in 0..1<<h{
    for b in 0..1<<w{
      let mut v=0;
      println!("a={:?}", a);
      println!("b={:?}", b);
      println!()

    }

  };
}