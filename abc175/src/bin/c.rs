fn main(){
 proconio::input!{mut x:i64,mut k:i64,d:i64};
 let mut x=x.abs();
 let s=k.min(x/d);
 k-=s;
 x-=s*d;
 let ans=if k%2==0{x}else{d-x};
 println!("{}",ans);
}