use proconio::input;

fn main() {
    input! {
        n: usize
    }
    // nを2進法に変換し、10桁の文字列とする。
    let binary = format!("{:010b}", n);
    println!("{}", binary);
}
