// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let option: Option<i32> = Some(12);
    let res = 42 + option.map_or(0, |x| x);
    println!("{}", res);
}
