fn main(){
  proconio::input!{n:String};
  println!("{}",if 0 ==n.chars().filter(|x|x.to_string()=="7").count(){"No"}else{"Yes"})
}