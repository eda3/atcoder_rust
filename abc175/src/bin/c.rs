fn main() {
    proconio::input!{mut x:i64,mut k:i64,d:i64};
    println!("x={} k={} d={}", x,k,d);
    // 移動回数と、x/d(初期位置から、移動距離を割った値)を比較
    let s=k.min(x/d);
    println!("s={:?}", s);
    k-=s;
    println!("k={:?}", k);
    x -= s*d;
    let ans = if k%2==0{x}else{d-x};
    println!("ans{:?}", ans);
}
