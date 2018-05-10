use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: u64 = iter.next().unwrap().parse().unwrap();
    let x: u64 = iter.next().unwrap().parse().unwrap();

    let lrt: Vec<u64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let sum: u64 = lrt.iter().sum();
    let min = lrt.iter().min().unwrap();

    let ans = n + (x - sum) / min;
    println!("{}", ans);
}
