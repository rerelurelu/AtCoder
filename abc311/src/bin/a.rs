use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut appeared = (false, false, false);

    for i in 0..n {
        match s[i] {
            'A' => appeared.0 = true,
            'B' => appeared.1 = true,
            'C' => appeared.2 = true,
            _ => break,
        }

        if appeared.0 && appeared.1 && appeared.2 {
            println!("{}", i + 1);
            return;
        }
    }
}
