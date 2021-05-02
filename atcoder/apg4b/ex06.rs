use proconio::input;

fn main() {
    input! {
        a: i32,
        op: String,
        b: i32
    }

    let ans;
    if op == "+" {
        ans = a+b;
    } else if op == "-" {
        ans = a-b;
    } else if op == "*" {
        ans = a*b;
    } else if op == "/" && b != 0 {
        ans = a/b;
    } else {
        println!("error");
        return;
    }
    println!("{}", ans);
}