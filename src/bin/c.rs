use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        d: [i64; n],
    }
    let mut dd: Vec<i64> = Vec::new();
    for di in d {
        dd.push((di % (a+b)).try_into().unwrap());
    }
    dd.sort_by_key(|&x| x);
    if (dd[dd.len()-1] - dd[0] < a) || (a + b - dd[1] + dd[0] < a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
