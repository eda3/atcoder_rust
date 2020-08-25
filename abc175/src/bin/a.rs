fn main(){
 proconio::input!{s:String};
 println!("{}",s.split("S").map(|x|x.len()).max().unwrap())
}