fn main(){
  proconio::input!{s:String};
  let l=s.len(); // 文字列の長さ
  let c=l/2; // 中心
  // println!("{}",&s[0..c]); // 先頭から中心まで
  // println!("{}",s[c+1..l]) // 中心から最後尾まで
  println!("{}",if &s[0..c]==&s[c+1..l]{"Yes"}else{"No"})

}