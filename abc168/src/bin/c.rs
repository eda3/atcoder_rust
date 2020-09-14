use std::f64::consts::PI;
fn main() {
  proconio::input!{a:f64,b:f64,h:f64,m:f64};
  let mut c=(h*30.+m*0.5).abs()-(m*6.);
  if 180.<=c{c=360.-c}
  println!("{}",(a*a+b*b-2.*a*b*c.to_radians().cos()).sqrt())
}