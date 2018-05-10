use std::io::Read;
fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 読み込んだStringを空白で分解する
    let mut iter = buf.split_whitespace();

    let a: u64 = iter.next().unwrap().parse().unwrap();
    let b: u64 = iter.next().unwrap().parse().unwrap();
    let c: u64 = iter.next().unwrap().parse().unwrap();

    let x: u64 = iter.next().unwrap().parse().unwrap();
    let y: u64 = iter.next().unwrap().parse().unwrap();

    let mut sum = a * x + b * y;

    let mut xa = x;
    let mut ya = y;
    let mut za = 0;

    loop {
        if xa != 0 {
            xa -= 1
        };
        if ya != 0 {
            ya -= 1
        };

        za += 2;

        let next = xa * a + ya * b + za * c;

        if sum <= next {
            break;
        } else {
            sum = next;
        }
    }

    println!("{}", sum);
}
