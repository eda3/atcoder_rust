fn main() {
  proconio::input!{n:u16};
  println!("{}on",match n%10{
    2|4|5|7|9 => "h",
    0|1|6|8 => "p",
    3 => "b",
    _ => ""
  });
}