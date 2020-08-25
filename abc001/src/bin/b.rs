use proconio::*;
fn main(){
    input!{m:isize}
    println!("{:02}",if m<100{0}else if m<=5000{m/100}else if m<=30000{m/1000+50}else if m<=70000{((m/1000)-30)/5+80}else{89});
}
