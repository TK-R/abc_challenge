use std::cmp;
use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ').map(|s| s.trim().parse().unwrap()).collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {
    let order = read_one::<String>();
    let mut ans = 700;
    for c in order.chars().collect::<Vec<char>>() {
        if c == 'o' {
            ans += 100;
        }
    }

    println!("{}", ans);
}
