use itertools::Itertools;
fn main() {
 proconio::input!{l:[u64]};
 println!("{:?}",l.into_iter().sorted().tuple_combinations().filter(|&(a,b,c)|a+b>c && a!=b && b!=c).count());
}