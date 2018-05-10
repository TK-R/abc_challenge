use std::io::{stdout, BufWriter, Write};
use std::cmp;

macro_rules! debug {
    ($($a:expr),*) => {
         eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

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


fn exam_a(){
    let s = get!(String);
    if s.contains("a") && s.contains("b") && s.contains("c") {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn exam_b(){
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let (A, B, K) = get!(u64, u64, u64);
    if B - A >= K*2 {
        for n in A..A+K {
            writeln!(out, "{}", n).unwrap();
        } 

        if B-K >= A+K {
            for n in B-K..B {
                writeln!(out, "{}", n+1).unwrap();
            }
        } else {
            for n in A+K..B {
                writeln!(out, "{}", n+1).unwrap();
            }
        }
    } else {
        for n in A..B+1 {
            writeln!(out, "{}", n).unwrap();
        }
    }
}

fn exam_c(){
    let (a,b,c) = get!(u32, u32, u32);
    let max = cmp::max(a, cmp::max(b, c));

    debug!(max);
    let tri_max = max * 3;
    let sum = a + b + c;

    let diff = tri_max - sum;
    let time =  if diff % 2 == 0    { diff / 2 }
                else                { (diff + 3) / 2 };
    
    println!("{}", time);


}

fn main() {
//    exam_a();
//    exam_b();
    exam_c();
}
