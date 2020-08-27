fn main() {
  proconio::input!{s:[String]};
  let mut m=std::collections::HashMap::new();
  for i in s{
    *m.entry(i).or_insert(0)+=1;
  }
  for s in &["AC","WA","TLE","RE"]{
    println!("{} x {}", s,m.get(&s.to_string()).unwrap_or(&0));
  }
}
