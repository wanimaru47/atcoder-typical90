use proconio::input;

fn dfs(p: String, lcnt: i32, rcnt: i32) {
    if lcnt == 0 && rcnt == 0 {
        println!("{}", p)
    }

    if lcnt > 0 {
        dfs(p.to_string() + "(", lcnt - 1, rcnt + 1);
    }
    if rcnt > 0 {
        dfs(p.to_string() + ")", lcnt, rcnt - 1)
    }
}

fn main() {
    input! {
        n: i32,
    }

    if n % 2 != 0 {
        return;
    }

    dfs("".to_string(), n / 2, 0);
}
