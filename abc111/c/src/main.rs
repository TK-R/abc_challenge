use std::collections::HashMap;
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

fn mode(vec: &Vec<u64>) -> u64 {
    let mut times = HashMap::new();

    // count
    for x in vec {
        let cnt = times.entry(*x as u64).or_insert(0);
        *cnt += 1;
    }

    let mut best: (u64, u64) = (
        *times.iter().nth(0).expect("Fatal.").0 as u64,
        *times.iter().nth(0).expect("Fatal.").1 as u64,
    );

    for x in times.iter() {
        if *x.1 > best.1 {
            best = (*x.0 as u64, *x.1);
        }
    }
    best.0
}

fn main() {
    // 数値
    let _n = {
        let i = read::<u64>();
        i[0]
    };
    let a = read::<u64>();

    let vec1 = a
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == 0)
        .map(|(_, &value)| value)
        .collect::<Vec<u64>>();

    let vec2 = a
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == 1)
        .map(|(_, &value)| value)
        .collect::<Vec<u64>>();

    let mode1 = mode(&vec1);
    let mode2 = mode(&vec2);

    if mode1 != mode2 {
        let mut ans = 0;

        for v in &vec1 {
            if *v != mode1 {
                ans += 1;
            }
        }

        for v in &vec2 {
            if *v != mode2 {
                ans += 1;
            }
        }

        println!("{}", ans);
    } else {
        let vec11 = vec1
            .iter()
            .enumerate()
            .filter(|&(_index, &digit)| digit != mode1)
            .map(|(_, &digit)| digit)
            .collect::<Vec<u64>>();

        let mut mode11 = if !vec11.is_empty() {
            mode(&vec11)
        } else {
            mode1
        };

        let vec22 = vec2
            .iter()
            .enumerate()
            .filter(|&(_index, &digit)| digit != mode2)
            .map(|(_, &digit)| digit)
            .collect::<Vec<u64>>();

        let mut mode22 = if !vec22.is_empty() {
            mode(&vec22)
        } else {
            mode2
        };

        if mode11 == mode22 {
            mode11 = 0;
            mode22 = 0;
        }

        let mut ans1 = 0;
        let mut ans2 = 0;

        for v in &vec1 {
            if *v != mode1 {
                ans1 += 1;
            }
        }

        for v in &vec2 {
            if *v != mode22 {
                ans1 += 1;
            }
        }

        for v in &vec1 {
            if *v != mode11 {
                ans2 += 1;
            }
        }

        for v in &vec2 {
            if *v != mode2 {
                ans2 += 1;
            }
        }
        // println!("{} {} {} {}", mode11, mode22, ans1, ans2);

        println!("{}", std::cmp::min(ans1, ans2));
    }
}
