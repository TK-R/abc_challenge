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

fn sunuke(n: String) -> f64 {
    let mut sn: f64 = 0.0;
    for c in n.chars() {
        sn += c.to_digit(10).unwrap() as f64;
    }
    sn
}

fn main() {
    let k = read::<u64>()[0];
    let mut count = 1;
    let mut out = 0;
    loop {
        let next = count + 1;
        let nextnext = count + 2;

        let sn = sunuke(count.to_string());
        let sna = sunuke(next.to_string());
        let snb = sunuke(nextnext.to_string());

        let x1 = count as f64 / sn;
        let x2 = next as f64 / sna;
        let x3 = nextnext as f64 / snb;

        if x1 <= x2 && x1 <= x3 {
            println!("{}", count);
            out += 1;
        }
        count += 1;

        if out >= k {
            break;
        }
    }
}
