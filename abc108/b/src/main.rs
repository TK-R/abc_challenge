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
    let (x1, y1, x2, y2) = {
        let i = read::<isize>();
        (i[0], i[1], i[2], i[3])
    };

    let mut x3;
    let mut x4;
    let mut y3;
    let mut y4;
    use std::cmp;

    let diffx = (x1 - x2).abs();
    let diffy = (y1 - y2).abs();

    if x1 < x2 && y1 <= y2 {
        x3 = x2 - diffy;
        y3 = y2 + diffx;
        
        x4 = x3 - diffx;
        y4 = y3 - diffy; 
    } else if y2 > y1 && x2 <= x1 {
        x3 = x2 - diffy;
        y3 = y2 - diffx;
        
        x4 = x3 + diffx;
        y4 = y3 - diffy; 
    } else if x1 > x2 {
        x3 = x2 + diffy;
        y3 = y2 - diffx;
        
        x4 = x3 + diffx;
        y4 = y3 + diffy; 
    } else  {
        x3 = x2 + diffy;
        y3 = y2 + diffx;
        
        x4 = x3 - diffx;
        y4 = y3 + diffy; 
    } 

    println!("{} {} {} {}", x3, y3, x4, y4);

}
