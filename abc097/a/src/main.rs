use std::io;

/// 一行を読み込んで、空白で区切ったベクタにして返す
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

/// n行を読み込んで、数字を空白で区切って二次元のベクタにして返す
#[allow(dead_code)]
fn read_table<T>(n: usize) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut ret: Vec<Vec<T>> = Vec::new();
    for _ in 0..n {
        ret.push(read::<T>());
    }
    ret
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

/// n行を読み込んで、一次元のベクタにして返す
#[allow(dead_code)]
fn read_array<T>(n: usize) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut array = Vec::new();

    for _ in 0..n {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        array.push(buf.trim().parse().unwrap());
    }
    array
}

fn main() {
    // 数値
    let (a, b, c, d) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    let ab = (a - b).abs() <= d;
    let ac = (a - c).abs() <= d;
    let bc = (b - c).abs() <= d;

    let mut ans = false;
    if ac {
        ans = true;
    }

    if ab && bc {
        ans = true;
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
