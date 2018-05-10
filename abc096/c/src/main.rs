use std::io::Read;

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
    let mut buf = String::new();

    let (h, w) = get!(usize, usize);
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.chars();

    let mut table: Vec<Vec<char>> = Vec::new();

    for _ in 0..h {
        let mut lrt: Vec<char> = (0..w + 1).map(|_| iter.next().unwrap()).collect();
        table.push(lrt);
    }

    let mut ans = "Yes";

    if h < 2 || w < 2 {
        if buf.contains('#') {
            println!("No");
        } else {
            println!("Yes");
        }
        return;
    }

    for i in 0..h {
        for j in 0..w {
            // 塗対象でなければ無視
            if table[i][j] == '.' {
                continue;
            }

            let up = if i != 0 { table[i - 1][j] } else { '.' };
            let down = if i != h - 1 { table[i + 1][j] } else { '.' };
            let left = if j != 0 { table[i][j - 1] } else { '.' };
            let right = if j != w - 1 { table[i][j + 1] } else { '.' };

            if up == '.' && down == '.' && left == '.' && right == '.' {
                ans = "No";
                break;
            }
        }
    }

    println!("{}", ans);
}
