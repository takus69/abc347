use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: HashSet<String> = HashSet::new();
    for i in 0..(s.len()) {
        for j in (i+1)..=(s.len()) {
            let ss: String = s[i..j].iter().collect();
            ans.insert(ss);
        }
    }
    println!("{}", ans.len());
}
