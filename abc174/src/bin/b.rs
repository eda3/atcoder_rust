fn main() {
    proconio::input!{n:u64,m:i64,l:[(i64,i64);n]}
    println!("{:?}", l.iter().filter(|(x,y)|x*x+y*y<=m*m).count());
}
