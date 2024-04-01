use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        if a[i] % k == 0 {
            ans.push(a[i]/k);
        }
    }
    println!("{}", ans.iter().map(|&s| s.to_string()).collect::<Vec<String>>().join(" "));
}
