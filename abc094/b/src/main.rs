
use std::io; // use宣言は関数内にも書けます
use std::str::FromStr;
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
   let expect = solve(5, 3, 3, vec![1, 2, 4]);   
   assert_eq!(1, expect);
}

#[test]
fn test_2(){
   let expect = solve(7, 3, 2, vec![4, 5, 6]);   
   assert_eq!(0, expect);
}

#[test]
fn test_3(){
   let expect = solve(10, 7, 5, vec![1, 2, 3, 4, 6, 8, 9]);   
   assert_eq!(3, expect);
}

fn solve(n: u32, m:u32, x:u32, mut ai:Vec<u32>) -> u32{
    ai.sort();
    let big     = ai.iter()
                    .enumerate()
                    .filter(|&(index, &digit)| digit > x)
                    .count();
    let small   = ai.iter()
                    .enumerate()
                    .filter(|&(index, &digit)| digit < x)
                    .count();

    cmp::min(big as u32, small as u32)
}

fn main() {

    let (n, m, x) = get!(u32, u32, u32);
    let mut ai = vec![1u32];
    
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();

    let ai: Vec<u32> = buf.split_whitespace()
        .map(|n| u32::from_str(n).unwrap())
        .collect();

    let ans = solve(n, m, x, ai);

    println!("{}", ans);   
    
}
