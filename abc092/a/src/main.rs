use std::cmp;

macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}

#[test]
fn test_1(){
   let expect = solve(600, 300, 220, 420);   
   assert_eq!(520, expect);
}

#[test]
fn test_2(){
   let expect = solve(555, 555, 400, 200);   
   assert_eq!(755, expect);
}

#[test]
fn test_3(){
   let expect = solve(549, 817, 715, 603);   
   assert_eq!(1152, expect);
}


fn solve(a: u32, b:u32, c:u32, d:u32) -> u32{
    let train = cmp::min(a, b);
    let bus = cmp::min(c, d);

    train + bus
}

fn main() {

    let a = get!(u32);
    let b = get!(u32);
    let c = get!(u32);
    let d = get!(u32);

    let ans = solve(a, b, c, d);

    println!("{}", ans);
}
