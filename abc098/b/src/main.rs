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

pub fn sum_from_zero(n: i32) -> i32 {
    (0..n + 1).fold(0, |a, b| a + b)
}

fn main() {
    // 数値
    let (a, b) = {
        let i = read::<i32>();
        (i[0], i[1])
    };

    let diff = b - a;
    let bh = sum_from_zero(diff);

    let h = bh - b;
    println!("{}", h);
}