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
    let n = {
        let i = read::<usize>();
        i[0]
    };

    let mut xi: Vec<isize> = Vec::new();
    let mut yi: Vec<isize> = Vec::new();
    let mut hi: Vec<isize> = Vec::new();

    for _i in 0..n {
        let vec = read::<isize>();
        let x = vec[0];
        let y = vec[1];
        let h = vec[2];

        xi.push(x);
        yi.push(y);
        hi.push(h);
    }

    for tx in 0..101 {
        for ty in 0..101 {
            let mut temph = 1;
            for i in 0..n {
                if hi[i] > 0 {
                    temph = hi[i] + (xi[i] - tx).abs() + (yi[i] - ty).abs();
                    break;
                }
            }

            let mut ansf = true;
            for i in 0..n {
                if hi[i] > 0 {
                    let h = hi[i] + (xi[i] - tx).abs() + (yi[i] - ty).abs();
                    if temph != h {
                        ansf = false;
                    }
                } else {
                    if temph - (xi[i] - tx).abs() - (yi[i] - ty).abs() > 0 {
                        ansf = false;
                    }
                }
            }

            if ansf == true {
                println!("{} {} {}", tx, ty, temph);
                return;
            }
        }
    }
}
