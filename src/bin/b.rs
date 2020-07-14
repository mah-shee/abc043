#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ans: String = String::new();
    for i in 0..s.len() {
        if s[i] == 'B' {
            ans.pop();
        } else {
            ans.push_str(&s[i].to_string());
        }
    }
    println!("{}", ans);
}
