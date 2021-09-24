use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        mut a: [i32; n],
    }

    a.push(l);

    let mut left = 0;
    let mut right = l;
    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut cnt = 0;
        let mut now = 0;
        for x in a.iter() {
            if now + mid < *x {
                cnt += 1;
                now = *x;
            }
        }

        if cnt > k {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", right);
}
