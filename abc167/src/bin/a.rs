fn main() {
  proconio::input!{s:String,t:String};
  println!("{}",if s==t[..(t.len()-1)]{"Yes"}else{"No"})
}