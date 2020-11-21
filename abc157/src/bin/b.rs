fn main() {
  proconio::input! {a:[i32;9],n:usize,b:[i32;n]};
  let mut x = [false; 9];
  for bk in b {
    for i in 0..9 {
      x[i] |= a[i] == bk;
    }
  }

  let lines = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
  ];
  let bingo = |x: [bool; 9]| {
    for line in &lines {
      if x[line[0]] && x[line[1]] && x[line[2]] {
        return true;
      }
    }
    return false;
  };
  println!("{}", if bingo(x) { "Yes" } else { "No" })
}