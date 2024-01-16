use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut plus = vec![0; d + 2];
    let mut minus = vec![0; d + 2];

    for (l, r) in lr {
        plus[l] += 1;
        minus[r + 1] += 1;
    }
    let mut now = 0;
    for i in 1..=d {
        now = now + plus[i] - minus[i];
        println!("{} ", now)
    }
}