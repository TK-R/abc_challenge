use std::io::Read;

macro_rules! debug {
    ($($a:expr),*) => {
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: u64 = iter.next().unwrap().parse().unwrap();
    let c: u64 = iter.next().unwrap().parse().unwrap();

    let mut lrt: Vec<(u64, u64)> = (0..n)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut cal: i64 = 0;
    let mut distance: u64 = 0;
    let mut pos: usize = 0;
    let mut count = 0;
    loop {
        let last = n as usize - pos - 1;

        let mut xcw = lrt[pos].0 as i64;
        let mut xccw = (c - lrt[last].0) as i64;

        let cal_cw = lrt[pos].1 as i64;
        let cal_ccw = lrt[last].1 as i64;

        debug!(cal_cw, cal_ccw);

        if cal_cw - xcw < 0 && cal_ccw - xccw < 0 {
            break;
        }

        if cal_cw - xcw >= cal_ccw - xccw {
            cal += cal_cw - xcw;
            lrt[pos].1 = 0;
            pos += 1;
        } else {
            cal += cal_ccw - xccw;
            pos = if pos != 0 { pos - 1 } else { n as usize }
        }

        debug!(cal, count, pos);
        count += 1;

        if count == n {
            break;
        }
    }
    println!("{}", cal);
}
