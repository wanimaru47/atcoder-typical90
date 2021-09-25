use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        v: [[i32; w]; h]
    }

    let mut sum_of_row = vec![0; h];
    let mut sum_of_column = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            sum_of_row[i] += v[i][j];
            sum_of_column[j] += v[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", sum_of_row[i] + sum_of_column[j] - v[i][j]);
        }
        println!();
    }
}
