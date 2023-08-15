use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    if n == 1 {
        println!("0");
    } else {
        let max = p.iter().skip(1).max().unwrap();

        match max.cmp(&p[0]) {
            std::cmp::Ordering::Less => println!("0"),
            std::cmp::Ordering::Equal => println!("1"),
            std::cmp::Ordering::Greater => println!("{}", max - p[0] + 1),
        }
    }
}
