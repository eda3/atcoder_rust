fn main(){
 proconio::input!{n:String}
 println!("{}",if n.chars().map(|x|x.to_digit(10).unwrap()).sum::<u32>()%9==0{"Yes"}else{"No"});
}
