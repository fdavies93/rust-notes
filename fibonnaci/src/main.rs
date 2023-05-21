// FIBONNACI SEQUENCE
// 1, 1, 2, 3, 5, 8, 13, 21, 34
// 0, 1, 2, 3
fn fibonnaci(n : i32) -> i32 {
    if n < 2 { return 1; }

    let mut n_less_1 = 1;
    let mut n_less_2 = 1;
    let mut current_n = 1;

    let mut i = 1;

    while i < n {
        current_n = n_less_1 + n_less_2;
        n_less_2 = n_less_1;
        n_less_1 = current_n;
        i += 1;
    }

    return current_n;
}

fn main() {
    let mut i = 0;
    while i < 20 {
        println!("{}",fibonnaci(i));
        i += 1;
    }
}
