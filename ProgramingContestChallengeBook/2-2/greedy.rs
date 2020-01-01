use std::cmp::min;

fn main() {
    let c_1: u32 = 3;
    let c_5: u32 = 2;
    let c_10: u32 = 1;
    let c_50: u32 = 3;
    let c_100: u32 = 0;
    let c_500: u32 = 2;
    let mut a: u32 = 620;
    let mut ans = 0;
    
    let v = vec![1, 5, 10, 50, 100, 500];
    let c = vec![c_1, c_5, c_10, c_50, c_100, c_500];
    for i in (5..0).rev() {
        let t = min(ans/v[i], c[i]);
        a -= t * v[i];
        ans += t;
    }
    println!("{}", ans);
}


