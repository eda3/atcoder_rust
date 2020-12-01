fn main(){
  proconio::input!{n:u32,m:usize,ss:[(usize, char);m]};
  println!("n{:?}",n);
  println!("sc{:?}",ss);
  let ans = (0..10i32.pow(n))
    .filter(|x| {
      let x = x.to_string().as_bytes().to_vec();
      x.len() == n as usize
      && ss
        .iter()
        .all(|(s,c)| matches!(x.get(*s-1), Some(x) if *x as char == *c))

    })
    .next()
    .unwrap_or(-1);
  dbg!(ans);
  println!("{:?}",ans);
}