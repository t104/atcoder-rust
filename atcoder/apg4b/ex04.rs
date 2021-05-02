fn main() {
    let seconds = 365*24*60*60;
    for y in &[1,2,5,10] {
        println!("{}", seconds*y);
    }
}