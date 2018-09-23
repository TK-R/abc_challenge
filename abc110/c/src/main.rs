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

    // 文字テーブル
    let mut buf1 = String::new();
    io::stdin().read_line(&mut buf1).unwrap();

    let sc = buf1.chars().collect::<Vec<char>>();
    let count = buf1.chars().count();

    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    let tc = buf2.chars().collect::<Vec<char>>();
    
    let mut c = Vec::<(char,char)>::with_capacity(1024);

    
    for i in 0..count {
        let mut  flag = true;
        for cc in &c {
            if cc.0 == sc[i] && cc.1 == tc[i] {
                flag = false;
            }
             
            if sc[i] != tc[i] {
                if cc.0 == sc[i] && cc.1 != tc[i] {
                    println!("No");
                    return;
                } else if cc.0 != sc[i] && cc.1 == tc[i] {
                    println!("No");
                    return;
                } 
            }

        }
        
        if flag { 
            c.push((sc[i], tc[i]));
        }
    }

    println!("Yes");

}