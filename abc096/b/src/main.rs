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

fn main() {
    let (_a, _b, _c) = get!(u64, u64, u64);
    let _k = get!(u32);

    let k: u32 = (_k).into();
    let base: u64 = 2;
    let mut a = _a;
    let mut b = _b;
    let mut c = _c;

    use std::cmp;

    if cmp::max(a, cmp::max(b, c)) == a {
        a = a * base.pow(k);
    } else if cmp::max(b, cmp::max(a, c)) == b {
        b = b * base.pow(k);
    } else {
        c = c * base.pow(k);
    }

    let ans = a + b + c;
    println!("{}", ans);
}
