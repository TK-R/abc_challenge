use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

/// n行を読み込んで、1文字ずつを二次元のベクタにして返す
#[allow(dead_code)]
fn read_char_table(n: usize) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let chr = buf.trim().chars().collect();
        ret.push(chr);
    }
    ret
}

fn main() {
    let (h, w) = {
        let i = read();
        (i[0], i[1])
    };

    let table: Vec<Vec<char>> = read_char_table(h);

    let mut ans = "Yes";

    if h < 2 || w < 2 {
        if table[0][0] == '#' {
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
