fn main(){
  proconio::input!{n:usize,a:usize,b:usize};
  println!("{}",(n/(a+b))*a+a.min(n%(a+b)))
}