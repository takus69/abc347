use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        C: i64,
    }

    let mut cnt = 0;
    for i in 0..60 {
        if (C>>i)&1 == 1 {
            cnt += 1;
        }
    }

    // println!("{} {} {}", a, b, cnt);
    if (a + b >= cnt) && ((a - b).abs() <= cnt) && ((a+b - cnt)%2 == 0) {
        let mut cnt2 = (a + b - cnt)/2;
        let mut na = a - cnt2;
        let mut nb = b - cnt2;
        let mut a_cnt = 0;
        let mut b_cnt = 0;
        let mut ans1: usize = 0;
        let mut ans2: usize = 0;
        for i in 0..60 {
            if (C>>(59-i))&1 == 1 {
                if na > 0 {
                    ans1 = (ans1+1)<<1;
                    ans2 = ans2<<1;
                    na -= 1;
                    a_cnt += 1;
                } else {
                    ans1 = ans1<<1;
                    ans2 = (ans2+1)<<1;
                    nb -= 1;
                    b_cnt += 1;
                }
            } else {
                if cnt2 > 0 {
                    ans1 = (ans1+1)<<1;
                    ans2 = (ans2+1)<<1;
                    cnt2 -= 1;
                    a_cnt += 1;
                    b_cnt += 1;
                } else {
                    ans1 = ans1<<1;
                    ans2 = ans2<<1;
                }
            }
        }
        ans1 = ans1>>1;
        ans2 = ans2>>1;
        let max_x = 2usize.pow(60);
        if (ans1 < max_x) && (ans2 < max_x) {
            println!("{} {}", ans1, ans2);
        } else {
            println!("{}", -1);
        }
        // println!("{} {} {} {}", a, b, a_cnt, b_cnt);
    } else {
        println!("{}", -1);
    }
}
