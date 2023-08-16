use proconio::input;

fn main() {
    input! {
        s: String,
    }

    match s.as_str() {
        "ACE" | "BDF" | "CEG" | "DFA" | "EGB" | "FAC" | "GBD" => println!("Yes"),
        _ => println!("No"),
    }
}
