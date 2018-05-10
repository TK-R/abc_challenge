
use std::io;
use std::str::FromStr;
use std::io::{stdout, Write, BufWriter};


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
   let l = vec![2, 4, 4, 3];
   let mut l_sort = l.clone();
   l_sort.sort();

   let expect = solve(4, 0, &l, &l_sort);
   assert_eq!(4, expect);
   let expect = solve(4, 1, &l, &l_sort);
   assert_eq!(3, expect);
   let expect = solve(4, 2, &l, &l_sort);
   assert_eq!(3, expect);
   let expect = solve(4, 3, &l, &l_sort);
   assert_eq!(4, expect);
}

#[test]
fn test_2(){
   let l = vec![1, 2];
   let mut l_sort = l.clone();
   l_sort.sort();

   let expect = solve(2, 0, &l, &l_sort);
   assert_eq!(2, expect);
   let expect = solve(2, 1, &l, &l_sort);
   assert_eq!(1, expect);
}


#[test]
fn test_3(){
    let l = vec![5, 5, 4, 4, 3, 3];
    let mut l_sort = l.clone();
    l_sort.sort();


    let expect = solve(6, 0, &l, &l_sort);
    assert_eq!(4, expect);   
    let expect = solve(6, 1, &l, &l_sort);
    assert_eq!(4, expect);
    let expect = solve(6, 2, &l, &l_sort);
    assert_eq!(4, expect);
    let expect = solve(6, 3, &l, &l_sort);
    assert_eq!(4, expect);
    let expect = solve(6, 4, &l, &l_sort);
    assert_eq!(4, expect);
    let expect = solve(6, 5, &l, &l_sort);
    assert_eq!(4, expect);

}

fn solve(n: u64, i:u64, xi:&Vec<u64>, xi_sort:&Vec<u64>) -> u64{
    let m1 = xi_sort[(n / 2 - 1) as usize];
    let m2 = xi_sort[(n / 2) as usize];
    if xi[i as usize] <= m1 {
        m2
    } else {
        m1
    }
}

fn main() {

    let n = get!(u64);
    
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();

    let xi: Vec<u64> = buf.split_whitespace()
        .map(|n| u64::from_str(n).unwrap())
        .collect();

    let mut xi_sort = xi.clone();
    xi_sort.sort();

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    
    for i in 0..n {
        let ans = solve(n, i, &xi, &xi_sort);
        writeln!(out, "{}", ans).unwrap();   
    }
}
