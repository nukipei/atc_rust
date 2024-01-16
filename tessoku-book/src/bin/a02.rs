use proconio::input;

fn main() {
    input! {
        n : usize,
        x : isize,
        a : [usize; n],
    }
    // 整数xが配列aに含まれているときYesを、含まれていないときNoを出力する
    if a.contains(&(x as usize)) {
        println!("Yes");
    } else {
        println!("No");
    }

}
